using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using System;
using System.Threading.Tasks;
using System.Linq;
using System.Collections.ObjectModel;
using System.Windows;
using System.Windows.Threading;
using WpfExample.Models;
using RustEtherNetIp;
using System.Threading;
using System.Collections.Generic;
using System.Diagnostics;

namespace WpfExample.ViewModels
{
    public partial class MainViewModel : ObservableObject
    {
        private EtherNetIpClient? _plcClient;
        private DispatcherTimer? _refreshTimer;
        private bool _isRefreshing;
        private readonly object _refreshLock = new();
        private const int MAX_RETRIES = 3;
        private const int RETRY_DELAY = 1000;
        private readonly SemaphoreSlim _tagOperationLock = new(1, 1);

        [ObservableProperty]
        private string plcAddress = "192.168.0.1:44818";

        [ObservableProperty]
        private bool isConnected;

        [ObservableProperty]
        private string connectionStatus = "Disconnected";

        [ObservableProperty]
        private string sessionId = "None";

        [ObservableProperty]
        private int readRate;

        [ObservableProperty]
        private int writeRate;

        [ObservableProperty]
        private string tagToDiscover = string.Empty;

        [ObservableProperty]
        private string tagName = string.Empty;

        [ObservableProperty]
        private string tagValue = string.Empty;

        [ObservableProperty]
        private string selectedDataType = "BOOL";

        // Batch Operations Properties
        [ObservableProperty]
        private string batchReadTags = "TestTag\nTestBool\nTestInt\nTestReal";

        [ObservableProperty]
        private string batchWriteTags = "TestTag=true\nTestBool=false\nTestInt=999\nTestReal=88.8";

        [ObservableProperty]
        private string batchResults = "";

        [ObservableProperty]
        private string batchPerformance = "";

        public ObservableCollection<string> DataTypes { get; } = new()
        {
            "BOOL",    // Boolean values
            "SINT",    // 8-bit signed integer (-128 to 127)
            "INT",     // 16-bit signed integer (-32,768 to 32,767)
            "DINT",    // 32-bit signed integer (-2.1B to 2.1B)
            "LINT",    // 64-bit signed integer
            "USINT",   // 8-bit unsigned integer (0 to 255)
            "UINT",    // 16-bit unsigned integer (0 to 65,535)
            "UDINT",   // 32-bit unsigned integer (0 to 4.3B)
            "ULINT",   // 64-bit unsigned integer
            "REAL",    // 32-bit IEEE 754 float
            "LREAL"    // 64-bit IEEE 754 double
            // Note: STRING and UDT types removed as they're not supported in current Rust library
        };

        public ObservableCollection<PlcTag> Tags { get; } = new();
        public ObservableCollection<string> LogMessages { get; } = new();

        public MainViewModel()
        {
            InitializeTags();
            SetupTimer();
        }

        private void InitializeTags()
        {
            // Updated test tags to match TypeScript frontend (remove STRING examples)
            // Focus on supported data types only
            Tags.Add(new PlcTag("TestTag", "BOOL"));
            Tags.Add(new PlcTag("TestBool", "BOOL"));
            Tags.Add(new PlcTag("TestInt", "DINT"));
            Tags.Add(new PlcTag("TestReal", "REAL"));
            
            // Additional supported integer types for advanced users
            Tags.Add(new PlcTag("TestSint", "SINT"));
            Tags.Add(new PlcTag("TestInt16", "INT"));
            Tags.Add(new PlcTag("TestLint", "LINT"));
            Tags.Add(new PlcTag("TestUsint", "USINT"));
            Tags.Add(new PlcTag("TestUint", "UINT"));
            Tags.Add(new PlcTag("TestUdint", "UDINT"));
            Tags.Add(new PlcTag("TestUlint", "ULINT"));
            Tags.Add(new PlcTag("TestLreal", "LREAL"));
            
            // Advanced tag addressing examples (for reference)
            Tags.Add(new PlcTag("Program:MainProgram.Motor.Status", "BOOL"));
            Tags.Add(new PlcTag("DataArray[5]", "DINT"));
            Tags.Add(new PlcTag("StatusWord.15", "BOOL"));
            Tags.Add(new PlcTag("MotorData.Speed", "REAL"));
            
            // Note: STRING tags are not included as they're not supported in current Rust library
        }

        private void SetupTimer()
        {
            _refreshTimer = new DispatcherTimer
            {
                Interval = TimeSpan.FromMilliseconds(100)
            };
            _refreshTimer.Tick += RefreshTimer_Tick;
        }

        [RelayCommand]
        private async Task ConnectAsync()
        {
            try
            {
                LogMessage("🔌 Connecting to PLC...");
                
                // Create and connect on background thread
                await Task.Run(() =>
                {
                    _plcClient = new EtherNetIpClient();
                    return _plcClient.Connect(PlcAddress);
                }).ContinueWith(t =>
                {
                    if (t.Result)
                    {
                        IsConnected = true;
                        ConnectionStatus = "Connected";
                        SessionId = $"0x{_plcClient?.ClientId:X8}";
                        
                        _refreshTimer?.Start();
                        LogMessage($"✅ Connected! Session ID: {SessionId}");
                    }
                    else
                    {
                        LogMessage("❌ Connection failed!");
                        _plcClient?.Dispose();
                        _plcClient = null;
                    }
                }, TaskScheduler.FromCurrentSynchronizationContext());
            }
            catch (Exception ex)
            {
                LogMessage($"❌ Connection error: {ex.Message}");
                _plcClient?.Dispose();
                _plcClient = null;
            }
        }

        [RelayCommand]
        private void Disconnect()
        {
            try
            {
                _refreshTimer?.Stop();
                
                _plcClient?.Dispose();
                _plcClient = null;
                
                IsConnected = false;
                ConnectionStatus = "Disconnected";
                SessionId = "None";
                
                // Clear tag values
                foreach (var tag in Tags)
                {
                    tag.Value = null;
                    tag.HasError = false;
                    tag.ErrorMessage = null;
                }
                
                LogMessage("📤 Disconnected from PLC");
            }
            catch (Exception ex)
            {
                LogMessage($"⚠️ Disconnect error: {ex.Message}");
            }
        }

        private async Task<T> RetryOperation<T>(Func<Task<T>> operation, string operationName)
        {
            for (int attempt = 0; attempt < MAX_RETRIES; attempt++)
            {
                try
                {
                    await _tagOperationLock.WaitAsync();
                    try
                    {
                        return await operation();
                    }
                    finally
                    {
                        _tagOperationLock.Release();
                    }
                }
                catch (Exception ex)
                {
                    if (attempt == MAX_RETRIES - 1)
                    {
                        LogMessage($"❌ {operationName} failed after {MAX_RETRIES} attempts: {ex.Message}");
                        throw;
                    }
                    LogMessage($"⚠️ {operationName} attempt {attempt + 1} failed: {ex.Message}");
                    await Task.Delay(RETRY_DELAY * (int)Math.Pow(2, attempt));
                }
            }
            throw new Exception($"{operationName} failed after {MAX_RETRIES} attempts");
        }

        [RelayCommand]
        private async Task DiscoverTag()
        {
            if (!IsConnected || _plcClient == null) return;

            try
            {
                LogMessage($"🔍 Discovering tag: {TagToDiscover}");
                
                await RetryOperation(async () =>
                {
                    // Run the synchronous PLC operations on a background thread
                    return await Task.Run(() =>
                    {
                        // Try to read the tag to determine its type - order matters for proper detection
                        try
                        {
                            var boolValue = _plcClient.ReadBool(TagToDiscover);
                            SelectedDataType = "BOOL";
                            TagName = TagToDiscover;
                            TagValue = boolValue.ToString();
                            LogMessage($"✅ Discovered BOOL tag: {TagToDiscover} = {boolValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var sintValue = _plcClient.ReadSint(TagToDiscover);
                            SelectedDataType = "SINT";
                            TagName = TagToDiscover;
                            TagValue = sintValue.ToString();
                            LogMessage($"✅ Discovered SINT tag: {TagToDiscover} = {sintValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var intValue = _plcClient.ReadInt(TagToDiscover);
                            SelectedDataType = "INT";
                            TagName = TagToDiscover;
                            TagValue = intValue.ToString();
                            LogMessage($"✅ Discovered INT tag: {TagToDiscover} = {intValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var dintValue = _plcClient.ReadDint(TagToDiscover);
                            SelectedDataType = "DINT";
                            TagName = TagToDiscover;
                            TagValue = dintValue.ToString();
                            LogMessage($"✅ Discovered DINT tag: {TagToDiscover} = {dintValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var lintValue = _plcClient.ReadLint(TagToDiscover);
                            SelectedDataType = "LINT";
                            TagName = TagToDiscover;
                            TagValue = lintValue.ToString();
                            LogMessage($"✅ Discovered LINT tag: {TagToDiscover} = {lintValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var usintValue = _plcClient.ReadUsint(TagToDiscover);
                            SelectedDataType = "USINT";
                            TagName = TagToDiscover;
                            TagValue = usintValue.ToString();
                            LogMessage($"✅ Discovered USINT tag: {TagToDiscover} = {usintValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var uintValue = _plcClient.ReadUint(TagToDiscover);
                            SelectedDataType = "UINT";
                            TagName = TagToDiscover;
                            TagValue = uintValue.ToString();
                            LogMessage($"✅ Discovered UINT tag: {TagToDiscover} = {uintValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var udintValue = _plcClient.ReadUdint(TagToDiscover);
                            SelectedDataType = "UDINT";
                            TagName = TagToDiscover;
                            TagValue = udintValue.ToString();
                            LogMessage($"✅ Discovered UDINT tag: {TagToDiscover} = {udintValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var ulintValue = _plcClient.ReadUlint(TagToDiscover);
                            SelectedDataType = "ULINT";
                            TagName = TagToDiscover;
                            TagValue = ulintValue.ToString();
                            LogMessage($"✅ Discovered ULINT tag: {TagToDiscover} = {ulintValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var realValue = _plcClient.ReadReal(TagToDiscover);
                            SelectedDataType = "REAL";
                            TagName = TagToDiscover;
                            TagValue = realValue.ToString();
                            LogMessage($"✅ Discovered REAL tag: {TagToDiscover} = {realValue}");
                            return true;
                        }
                        catch { }

                        try
                        {
                            var lrealValue = _plcClient.ReadLreal(TagToDiscover);
                            SelectedDataType = "LREAL";
                            TagName = TagToDiscover;
                            TagValue = lrealValue.ToString();
                            LogMessage($"✅ Discovered LREAL tag: {TagToDiscover} = {lrealValue}");
                            return true;
                        }
                        catch { }

                        LogMessage($"❌ Could not determine type for tag: {TagToDiscover}");
                        return false;
                    });
                }, "Tag discovery");
            }
            catch (Exception ex)
            {
                LogMessage($"❌ Discovery error: {ex.Message}");
            }
        }

        [RelayCommand]
        private async Task ReadTag()
        {
            if (!IsConnected || _plcClient == null) return;

            try
            {
                LogMessage($"📖 Reading tag: {TagName}");
                
                await RetryOperation(async () =>
                {
                    // Run the synchronous PLC operations on a background thread
                    return await Task.Run(() =>
                    {
                        object value = SelectedDataType switch
                        {
                            "BOOL" => _plcClient.ReadBool(TagName),
                            "SINT" => _plcClient.ReadSint(TagName),
                            "INT" => _plcClient.ReadInt(TagName),
                            "DINT" => _plcClient.ReadDint(TagName),
                            "LINT" => _plcClient.ReadLint(TagName),
                            "USINT" => _plcClient.ReadUsint(TagName),
                            "UINT" => _plcClient.ReadUint(TagName),
                            "UDINT" => _plcClient.ReadUdint(TagName),
                            "ULINT" => _plcClient.ReadUlint(TagName),
                            "REAL" => _plcClient.ReadReal(TagName),
                            "LREAL" => _plcClient.ReadLreal(TagName),
                            "STRING" => _plcClient.ReadString(TagName),
                            "UDT" => _plcClient.ReadUdt(TagName),
                            _ => throw new Exception($"Unsupported data type: {SelectedDataType}")
                        };
                        
                        TagValue = value.ToString() ?? string.Empty;
                        LogMessage($"✅ Read {SelectedDataType} tag: {TagName} = {value}");
                        return true;
                    });
                }, "Tag read");
            }
            catch (Exception ex)
            {
                LogMessage($"❌ Read error: {ex.Message}");
            }
        }

        [RelayCommand]
        private void WriteTag()
        {
            if (!IsConnected || _plcClient == null) return;

            try
            {
                LogMessage($"✏️ Writing tag: {TagName}");
                
                switch (SelectedDataType)
                {
                    case "BOOL":
                        if (bool.TryParse(TagValue, out bool boolValue))
                        {
                            _plcClient.WriteBool(TagName, boolValue);
                            LogMessage($"✅ Wrote BOOL tag: {TagName} = {boolValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid boolean value");
                        }
                        break;
                        
                    case "SINT":
                        if (sbyte.TryParse(TagValue, out sbyte sintValue))
                        {
                            _plcClient.WriteSint(TagName, sintValue);
                            LogMessage($"✅ Wrote SINT tag: {TagName} = {sintValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid SINT value (-128 to 127)");
                        }
                        break;
                        
                    case "INT":
                        if (short.TryParse(TagValue, out short intValue))
                        {
                            _plcClient.WriteInt(TagName, intValue);
                            LogMessage($"✅ Wrote INT tag: {TagName} = {intValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid INT value (-32,768 to 32,767)");
                        }
                        break;
                        
                    case "DINT":
                        if (int.TryParse(TagValue, out int dintValue))
                        {
                            _plcClient.WriteDint(TagName, dintValue);
                            LogMessage($"✅ Wrote DINT tag: {TagName} = {dintValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid DINT value");
                        }
                        break;
                        
                    case "LINT":
                        if (long.TryParse(TagValue, out long lintValue))
                        {
                            _plcClient.WriteLint(TagName, lintValue);
                            LogMessage($"✅ Wrote LINT tag: {TagName} = {lintValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid LINT value");
                        }
                        break;
                        
                    case "USINT":
                        if (byte.TryParse(TagValue, out byte usintValue))
                        {
                            _plcClient.WriteUsint(TagName, usintValue);
                            LogMessage($"✅ Wrote USINT tag: {TagName} = {usintValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid USINT value (0 to 255)");
                        }
                        break;
                        
                    case "UINT":
                        if (ushort.TryParse(TagValue, out ushort uintValue))
                        {
                            _plcClient.WriteUint(TagName, uintValue);
                            LogMessage($"✅ Wrote UINT tag: {TagName} = {uintValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid UINT value (0 to 65,535)");
                        }
                        break;
                        
                    case "UDINT":
                        if (uint.TryParse(TagValue, out uint udintValue))
                        {
                            _plcClient.WriteUdint(TagName, udintValue);
                            LogMessage($"✅ Wrote UDINT tag: {TagName} = {udintValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid UDINT value");
                        }
                        break;
                        
                    case "ULINT":
                        if (ulong.TryParse(TagValue, out ulong ulintValue))
                        {
                            _plcClient.WriteUlint(TagName, ulintValue);
                            LogMessage($"✅ Wrote ULINT tag: {TagName} = {ulintValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid ULINT value");
                        }
                        break;
                        
                    case "REAL":
                        if (float.TryParse(TagValue, out float realValue))
                        {
                            _plcClient.WriteReal(TagName, realValue);
                            LogMessage($"✅ Wrote REAL tag: {TagName} = {realValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid REAL value");
                        }
                        break;
                        
                    case "LREAL":
                        if (double.TryParse(TagValue, out double lrealValue))
                        {
                            _plcClient.WriteLreal(TagName, lrealValue);
                            LogMessage($"✅ Wrote LREAL tag: {TagName} = {lrealValue}");
                        }
                        else
                        {
                            throw new Exception("Invalid LREAL value");
                        }
                        break;
                        
                    case "STRING":
                        _plcClient.WriteString(TagName, TagValue);
                        LogMessage($"✅ Wrote STRING tag: {TagName} = '{TagValue}'");
                        break;
                        
                    case "UDT":
                        LogMessage("❌ UDT writing not supported in this example");
                        break;
                        
                    default:
                        throw new Exception($"Unsupported data type: {SelectedDataType}");
                }
            }
            catch (Exception ex)
            {
                LogMessage($"❌ Write error: {ex.Message}");
            }
        }

        [RelayCommand]
        private async Task RunBenchmarkAsync()
        {
            if (!IsConnected || _plcClient == null) return;

            try
            {
                LogMessage("📊 Running benchmark...");
                
                var startTime = DateTime.Now;
                var readCount = 0;
                var writeCount = 0;
                
                // Run benchmark on background thread
                await Task.Run(() =>
                {
                    while ((DateTime.Now - startTime).TotalSeconds < 5)
                    {
                        try
                        {
                            _plcClient?.ReadBool("TestTag");
                            readCount++;
                        }
                        catch { }
                        
                        try
                        {
                            _plcClient?.WriteBool("TestTag", true);
                            writeCount++;
                        }
                        catch { }
                    }
                });
                
                ReadRate = (int)(readCount / 5.0);
                WriteRate = (int)(writeCount / 5.0);
                
                LogMessage($"✅ Benchmark complete: {ReadRate} reads/sec, {WriteRate} writes/sec");
            }
            catch (Exception ex)
            {
                LogMessage($"❌ Benchmark error: {ex.Message}");
            }
        }

        [RelayCommand]
        private async Task CreateTestTags()
        {
            if (!IsConnected || _plcClient == null) return;

            try
            {
                LogMessage("📋 Creating test tags in PLC...");

                var testTags = new(string name, string type, object value)[]
                {
                    ("TestTag", "BOOL", true),
                    ("TestBool", "BOOL", false),
                    ("TestInt", "DINT", 42),
                    ("TestReal", "REAL", 123.45f)
                };

                int successCount = 0;
                int errorCount = 0;

                foreach (var (name, type, value) in testTags)
                {
                    try
                    {
                        await Task.Run(() =>
                        {
                            switch (type)
                            {
                                case "BOOL":
                                    _plcClient.WriteBool(name, (bool)value);
                                    break;
                                case "DINT":
                                    _plcClient.WriteDint(name, (int)value);
                                    break;
                                case "REAL":
                                    _plcClient.WriteReal(name, (float)value);
                                    break;
                            }
                        });

                        LogMessage($"✅ Created {type} tag: {name} = {value}");
                        successCount++;
                    }
                    catch (Exception ex)
                    {
                        LogMessage($"❌ Failed to create {name}: {ex.Message}");
                        errorCount++;
                    }
                }

                if (successCount > 0)
                {
                    LogMessage($"✅ Created {successCount}/{testTags.Length} test tags successfully");
                    LogMessage("🚀 Test tags are ready for operations!");
                }
                else
                {
                    LogMessage($"❌ Failed to create any test tags ({errorCount} errors)");
                }
            }
            catch (Exception ex)
            {
                LogMessage($"❌ Error creating test tags: {ex.Message}");
            }
        }

        [RelayCommand]
        private async Task ExecuteBatchRead()
        {
            if (!IsConnected || _plcClient == null) return;

            try
            {
                var tagNames = BatchReadTags.Split('\n')
                    .Select(line => line.Trim())
                    .Where(line => !string.IsNullOrEmpty(line))
                    .ToArray();

                if (tagNames.Length == 0)
                {
                    LogMessage("❌ Please enter at least one tag name for batch read");
                    return;
                }

                LogMessage($"🚀 Executing batch read for {tagNames.Length} tags...");
                var stopwatch = System.Diagnostics.Stopwatch.StartNew();

                var results = await Task.Run(() => _plcClient.ReadTagsBatch(tagNames));
                
                stopwatch.Stop();
                var totalTime = stopwatch.ElapsedMilliseconds;

                var resultsText = new System.Text.StringBuilder();
                int successCount = 0;

                resultsText.AppendLine($"📊 Batch Read Results ({tagNames.Length} tags in {totalTime}ms):");
                resultsText.AppendLine("".PadRight(50, '='));

                foreach (var result in results)
                {
                    if (result.Value.Success)
                    {
                        resultsText.AppendLine($"✅ {result.Key}: {result.Value.Value} ({result.Value.DataType})");
                        successCount++;
                    }
                    else
                    {
                        resultsText.AppendLine($"❌ {result.Key}: {result.Value.ErrorMessage}");
                    }
                }

                BatchResults = resultsText.ToString();
                BatchPerformance = $"⏱️ Performance: {totalTime}ms total, {(double)totalTime / tagNames.Length:F1}ms avg/tag, {successCount}/{tagNames.Length} successful";
                
                LogMessage($"✅ Batch read completed: {successCount}/{tagNames.Length} successful in {totalTime}ms");
            }
            catch (Exception ex)
            {
                LogMessage($"❌ Batch read error: {ex.Message}");
                BatchResults = $"❌ Batch read failed: {ex.Message}";
                BatchPerformance = "⏱️ Performance: Error occurred";
            }
        }

        [RelayCommand]
        private async Task ExecuteBatchWrite()
        {
            if (!IsConnected || _plcClient == null) return;

            try
            {
                var lines = BatchWriteTags.Split('\n')
                    .Select(line => line.Trim())
                    .Where(line => !string.IsNullOrEmpty(line) && line.Contains('='))
                    .ToArray();

                if (lines.Length == 0)
                {
                    LogMessage("❌ Please enter tag=value pairs (one per line) for batch write");
                    return;
                }

                var tagValues = new Dictionary<string, object>();

                foreach (var line in lines)
                {
                    var parts = line.Split('=', 2);
                    if (parts.Length == 2)
                    {
                        var tagName = parts[0].Trim();
                        var valueStr = parts[1].Trim();

                        // Try to parse the value as different types
                        object value;
                        if (bool.TryParse(valueStr, out bool boolVal))
                            value = boolVal;
                        else if (int.TryParse(valueStr, out int intVal))
                            value = intVal;
                        else if (float.TryParse(valueStr, out float floatVal))
                            value = floatVal;
                        else
                            value = valueStr; // String

                        tagValues[tagName] = value;
                    }
                }

                LogMessage($"✏️ Executing batch write for {tagValues.Count} tags...");
                var stopwatch = System.Diagnostics.Stopwatch.StartNew();

                var results = await Task.Run(() => _plcClient.WriteTagsBatch(tagValues));
                
                stopwatch.Stop();
                var totalTime = stopwatch.ElapsedMilliseconds;

                var resultsText = new System.Text.StringBuilder();
                int successCount = 0;

                resultsText.AppendLine($"✏️ Batch Write Results ({tagValues.Count} tags in {totalTime}ms):");
                resultsText.AppendLine("".PadRight(50, '='));

                foreach (var result in results)
                {
                    var originalValue = tagValues.ContainsKey(result.Key) ? tagValues[result.Key] : "Unknown";
                    
                    if (result.Value.Success)
                    {
                        resultsText.AppendLine($"✅ {result.Key}: {originalValue} → Written successfully");
                        successCount++;
                    }
                    else
                    {
                        resultsText.AppendLine($"❌ {result.Key}: {originalValue} → {result.Value.ErrorMessage}");
                    }
                }

                BatchResults = resultsText.ToString();
                BatchPerformance = $"⏱️ Performance: {totalTime}ms total, {(double)totalTime / tagValues.Count:F1}ms avg/tag, {successCount}/{tagValues.Count} successful";
                
                LogMessage($"✅ Batch write completed: {successCount}/{tagValues.Count} successful in {totalTime}ms");
            }
            catch (Exception ex)
            {
                LogMessage($"❌ Batch write error: {ex.Message}");
                BatchResults = $"❌ Batch write failed: {ex.Message}";
                BatchPerformance = "⏱️ Performance: Error occurred";
            }
        }

        private async void RefreshTimer_Tick(object? sender, EventArgs e)
        {
            if (!IsConnected || _plcClient == null || _isRefreshing) return;

            lock (_refreshLock)
            {
                if (_isRefreshing) return;
                _isRefreshing = true;
            }

            try
            {
                // Read all tags in parallel on background thread
                await Task.Run(() =>
                {
                    Parallel.ForEach(Tags, tag =>
                    {
                        try
                        {
                            object value = tag.DataType switch
                            {
                                "BOOL" => _plcClient?.ReadBool(tag.Name) ?? false,
                                "DINT" => _plcClient?.ReadDint(tag.Name) ?? 0,
                                "REAL" => _plcClient?.ReadReal(tag.Name) ?? 0.0f,
                                _ => "Unknown"
                            };
                            
                            Application.Current.Dispatcher.Invoke(() => tag.UpdateValue(value));
                        }
                        catch (Exception ex)
                        {
                            Application.Current.Dispatcher.Invoke(() => tag.SetError(ex.Message));
                        }
                    });
                });
            }
            catch (Exception ex)
            {
                LogMessage($"⚠️ Refresh error: {ex.Message}");
            }
            finally
            {
                _isRefreshing = false;
            }
        }

        private void LogMessage(string message)
        {
            var timestamp = DateTime.Now.ToString("HH:mm:ss");
            var logEntry = $"[{timestamp}] {message}";
            
            Application.Current.Dispatcher.Invoke(() =>
            {
                LogMessages.Insert(0, logEntry);
                
                // Keep only last 100 messages
                while (LogMessages.Count > 100)
                {
                    LogMessages.RemoveAt(LogMessages.Count - 1);
                }
            });
        }
    }
}