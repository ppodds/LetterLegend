using System;
using System.IO;
using System.Net.Sockets;
using System.Threading;
using System.Threading.Tasks;
using Google.Protobuf;
using Protos.Control;

namespace IO.Net
{
    public class GameTcpClient
    {
        private readonly string _host;
        private readonly int _port;

        private readonly TcpClient _client;

        public GameTcpClient(string host, int port)
        {
            _host = host;
            _port = port;

            _client = new TcpClient();
        }

        public async Task ConnectAsync(string name)
        {
            await _client.ConnectAsync(_host, _port);
            var req = new ConnectRequest()
            {
                Name = name
            };
            var stream = new MemoryStream();
            req.WriteTo(new CodedOutputStream(stream));
                
            var res = ConnectResponse.Parser.ParseFrom(await Rpc(Operation.Connect, stream.ToArray()));
            if (!res.Success)
            {
                throw new Exception("create player failed");
            }
        }

        public Task Reconnect()
        {
            throw new NotImplementedException();
        }

        public async Task Disconnect()
        {
            var res = DisconnectResponse.Parser.ParseFrom(await Rpc(Operation.Disconnect));
            if (!res.Success)
            {
                throw new Exception("disconnect failed");
            }
            _client.Close();
        }
        
        private async Task<byte[]> Rpc(Operation operation, bool readResponse = true)
        {
            return await Rpc(operation, Array.Empty<byte>(), readResponse);
        }

        private async Task<byte[]> Rpc(Operation operation, byte[] data, bool readResponse = true,
            CancellationToken token = default)
        {
            await RpcCall(operation, data);
            var result = readResponse ? await ReadRpcResponse(token) : null;
            return result;
        }

        private async Task RpcCall(Operation operation, byte[] data)
        {
            var stream = _client.GetStream();
            var outputStream = new MemoryStream();
            await outputStream.WriteAsync(new byte[] { (byte)operation, 0, 0, 0 });
            await outputStream.WriteAsync(BitConverter.GetBytes(data.Length));
            await outputStream.WriteAsync(data);
            await stream.WriteAsync(outputStream.ToArray());
        }

        private async Task<byte[]> ReadRpcResponse(CancellationToken token = default)
        {
            var stream = _client.GetStream();
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