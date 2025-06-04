using System;
using System.Collections.Generic;

namespace RustEtherNetIp
{
    /// <summary>
    /// Interface for EtherNet/IP client supporting Allen-Bradley CompactLogix and ControlLogix PLCs.
    /// Provides comprehensive data type support, advanced tag addressing capabilities, and high-performance batch operations.
    /// </summary>
    public interface IEtherNetIpClient : IDisposable
    {
        // Connection Management
        bool Connect(string address);
        void Disconnect();
        bool IsConnected { get; }
        int ClientId { get; }
        
        // Boolean Operations
        bool ReadBool(string tagName);
        void WriteBool(string tagName, bool value);
        
        // Signed Integer Operations
        sbyte ReadSint(string tagName);
        void WriteSint(string tagName, sbyte value);
        
        short ReadInt(string tagName);
        void WriteInt(string tagName, short value);
        
        int ReadDint(string tagName);
        void WriteDint(string tagName, int value);
        
        long ReadLint(string tagName);
        void WriteLint(string tagName, long value);
        
        // Unsigned Integer Operations
        byte ReadUsint(string tagName);
        void WriteUsint(string tagName, byte value);
        
        ushort ReadUint(string tagName);
        void WriteUint(string tagName, ushort value);
        
        uint ReadUdint(string tagName);
        void WriteUdint(string tagName, uint value);
        
        ulong ReadUlint(string tagName);
        void WriteUlint(string tagName, ulong value);
        
        // Floating Point Operations
        float ReadReal(string tagName);
        void WriteReal(string tagName, float value);
        
        double ReadLreal(string tagName);
        void WriteLreal(string tagName, double value);
        
        // String Operations
        string ReadString(string tagName);
        void WriteString(string tagName, string value);
        
        // UDT Operations
        Dictionary<string, object> ReadUdt(string tagName);
        void WriteUdt(string tagName, Dictionary<string, object> value);
        
        // Batch Operations - High Performance Multi-Tag Operations
        
        /// <summary>
        /// Read multiple tags in a single optimized batch operation.
        /// Provides 3-10x performance improvement over individual reads.
        /// </summary>
        /// <param name="tagNames">Array of tag names to read</param>
        /// <returns>Dictionary of tag names to read results</returns>
        Dictionary<string, TagReadResult> ReadTagsBatch(string[] tagNames);
        
        /// <summary>
        /// Write multiple tags in a single optimized batch operation.
        /// Provides 3-10x performance improvement over individual writes.
        /// </summary>
        /// <param name="tagValues">Dictionary of tag names to values to write</param>
        /// <returns>Dictionary of tag names to write results</returns>
        Dictionary<string, TagWriteResult> WriteTagsBatch(Dictionary<string, object> tagValues);
        
        /// <summary>
        /// Execute a mixed batch of read and write operations in optimized packets.
        /// Ideal for coordinated control operations and data collection.
        /// </summary>
        /// <param name="operations">Array of batch operations to execute</param>
        /// <returns>Array of batch operation results</returns>
        BatchOperationResult[] ExecuteBatch(BatchOperation[] operations);
        
        /// <summary>
        /// Configure batch operation behavior for performance optimization.
        /// </summary>
        /// <param name="config">Batch configuration settings</param>
        void ConfigureBatchOperations(BatchConfig config);
        
        /// <summary>
        /// Get current batch operation configuration.
        /// </summary>
        /// <returns>Current batch configuration</returns>
        BatchConfig GetBatchConfig();
        
        // Tag Management
        void DiscoverTags();
        TagMetadata GetTagMetadata(string tagName);
        
        // Configuration
        void SetMaxPacketSize(int size);
        bool CheckHealth();
    }
} 