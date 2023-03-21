using System;
using System.IO;
using System.Net.Sockets;
using System.Threading;
using System.Threading.Tasks;

namespace IO.Net
{
    public class GameTcpClient
    {
        private readonly string _host;
        private readonly int _port;

        public GameTcpClient(string host, int port)
        {
            _host = host;
            _port = port;
        }

        private async Task<byte[]> Rpc(byte procId, bool readResponse = true)
        {
            return await Rpc(procId, Array.Empty<byte>(), readResponse);
        }

        private async Task<byte[]> Rpc(byte procId, byte[] data, bool readResponse = true,
            CancellationToken token = default)
        {
            var client = new TcpClient();
            await client.ConnectAsync(_host, _port);
            await RpcCall(client, procId, data);
            var result = readResponse ? await ReadRpcResponse(client, token) : null;
            client.Close();
            client.Dispose();
            return result;
        }

        private static async Task RpcCall(TcpClient client, byte procId, byte[] data)
        {
            var stream = client.GetStream();
            var outputStream = new MemoryStream();
            await outputStream.WriteAsync(new[] { procId });
            await outputStream.WriteAsync(BitConverter.GetBytes(data.Length));
            await outputStream.WriteAsync(data);
            await stream.WriteAsync(outputStream.ToArray());
        }

        private static async Task<byte[]> ReadRpcResponse(TcpClient client, CancellationToken token = default)
        {
            var stream = client.GetStream();
            var buf = new byte[4];
            var n = await stream.ReadAsync(buf, token);
            token.ThrowIfCancellationRequested();
            if (n != buf.Length)
                throw new WrongProtocolException();
            var resLength = BitConverter.ToUInt32(buf);
            if (resLength == 0)
                return Array.Empty<byte>();
            buf = new byte[resLength];
            n = await stream.ReadAsync(buf, token);
            token.ThrowIfCancellationRequested();
            if (n != buf.Length)
                throw new WrongProtocolException();
            return buf;
        }
    }
}