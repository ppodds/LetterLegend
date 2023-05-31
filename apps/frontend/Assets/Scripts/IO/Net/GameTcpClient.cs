using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net.Sockets;
using System.Threading;
using System.Threading.Tasks;
using Google.Protobuf;
using Protos.Control;
using Protos.Game;
using Protos.Lobby;
using Unity.VisualScripting;
using UnityEngine;

namespace IO.Net
{
    public class GameTcpClient
    {
        private readonly string _host;
        private readonly int _port;
        private readonly TcpClient _client;
        private readonly Task _thread;
        private readonly Dictionary<uint, TaskCompletionSource<byte[]>> _taskMap;

        public GameTcpClient(string host, int port)
        {
            _host = host;
            _port = port;
            _client = new TcpClient();
            _taskMap = new Dictionary<uint, TaskCompletionSource<byte[]>>();
        }
        
        private async void Loop()
        {
            await Task.Run(async () =>
            {
                var stream = _client.GetStream();
                while (true)
                {
                    try
                    {
                        // read state
                        var buf = new byte[4];
                        Debug.Log("stuck read");
                        var n = await stream.ReadAsync(buf);
                        if (n != buf.Length)
                            throw new WrongProtocolException();
                        var state = BitConverter.ToUInt32(buf);
                        Debug.Log(state);
                        // read length
                        buf = new byte[4];
                        n = await stream.ReadAsync(buf);
                        if (n != buf.Length)
                            throw new WrongProtocolException();
                        var resLength = BitConverter.ToUInt32(buf);
                        if (resLength == 0)
                            return Array.Empty<byte>();
                        // read data
                        buf = new byte[resLength];
                        n = await stream.ReadAsync(buf);
                        if (n != buf.Length)
                            throw new WrongProtocolException();
                        _taskMap[state].SetResult(buf);
                    }
                    catch (Exception ex)
                    {
                        Debug.LogException(ex);
                    }
                }
            });
        }

        public async Task ConnectAsync(string name)
        {
            await _client.ConnectAsync(_host, _port);
            Loop();
            var req = new ConnectRequest()
            {
                Name = name
            };
            var stream = new MemoryStream();
            req.WriteTo(stream);

            // var responseTaskCompletionSource = new TaskCompletionSource<byte[]>();
            // _taskMap.Add(0, responseTaskCompletionSource);
            // await RpcCall(Operation.Connect, stream.ToArray());
            // var buf = await WaitForResponse(0);
            // var res = ConnectResponse.Parser.ParseFrom(buf);
            
            var res = ConnectResponse.Parser.ParseFrom(await RpcTest(Operation.Connect, stream.ToArray()));
            Debug.Log(res);
            if (!res.Success)
            {
                throw new Exception("create player failed");
            }
        }

        public async Task<List<LobbyInfo>> GetLobbies()
        {
            var res = ListResponse.Parser.ParseFrom(await Rpc(Operation.ListLobby));
            if (!res.Success)
            {
                throw new Exception("get lobby list fail");
            }

            return res.LobbyInfos.LobbyInfos_.ToList();
        }

        public async Task<Lobby> CreateLobby(uint maxPlayers)
        {
            var req = new CreateRequest()
            {
                MaxPlayers = maxPlayers
            };

            var stream = new MemoryStream();
            req.WriteTo(stream);
            var res = CreateResponse.Parser.ParseFrom(await Rpc(Operation.CreateLobby, stream.ToArray()));
            if (!res.Success)
            {
                throw new Exception("create room failed");
            }

            return res.Lobby;
        }

        public async Task<Lobby> JoinLobby(uint lobbyId)
        {
            var req = new JoinRequest()
            {
                LobbyId = lobbyId
            };

            var stream = new MemoryStream();
            req.WriteTo(stream);
            var res = JoinResponse.Parser.ParseFrom(await Rpc(Operation.JoinLobby, stream.ToArray()));
            if (!res.Success)
            {
                throw new Exception("join room failed");
            }

            return res.Lobby;
        }

        public async Task QuitLobby()
        {
            var res = QuitResponse.Parser.ParseFrom(await Rpc(Operation.QuitLobby));
            if (!res.Success)
            {
                throw new Exception("Quit lobby failed");
            }
        }

        public async Task<bool> SetReady()
        {
            var res = ReadyResponse.Parser.ParseFrom(await Rpc(Operation.Ready));
            if (!res.Success)
            {
                throw new Exception("Set Ready failed");
            }

            return true;
        }

        public async Task<Protos.Game.Board> Start()
        {
            var res = StartResponse.Parser.ParseFrom(await Rpc(Operation.StartGame));
            if (!res.Success)
            {
                throw new Exception("Someone is not Ready");
            }

            return res.Board;
        }

        public async Task<bool> SetTile(uint x, uint y, uint cardIndex)
        {
            var req = new SetTileRequest()
            {
                X = x,
                Y = y,
                CardIndex = cardIndex
            };

            var stream = new MemoryStream();
            req.WriteTo(stream);

            var res = SetTileResponse.Parser.ParseFrom(await Rpc(Operation.SetTile, stream.ToArray()));
            if (!res.Success)
            {
                throw new Exception("set tile failed");
            }

            return res.Success;
        }

        public async Task<List<Card>> GetNewCard()
        {
            var res = GetNewCardResponse.Parser.ParseFrom(await Rpc(Operation.GetNewCard));
            if (!res.Success)
            {
                throw new Exception("get new card failed");
            }
            return res.Cards.ToList();
        }

        public async Task FinishTurn()
        {
            var res = FinishTurnResponse.Parser.ParseFrom(await Rpc(Operation.FinishTurn));
            if (!res.Success)
            {
                throw new Exception("finish turn failed");
            }
        }

        public async Task HeartBeat()
        {
            var res = HeartbeatResponse.Parser.ParseFrom(await Rpc(Operation.Heartbeat));
            if (!res.Success)
            {
                throw new Exception("heart beat failed");
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

        public async Task<byte[]> Rpc(Operation operation, byte[] data, bool readResponse = true,
            CancellationToken token = default)
        {
            await RpcCall(operation, data);
            var result = readResponse ? await ReadRpcResponse(token) : null;
            return result;
        }
        
        public async Task<byte[]> RpcTest(Operation operation, byte[] data, bool readResponse = true)
        {
            var responseTaskCompletionSource = new TaskCompletionSource<byte[]>();
            _taskMap.Add(0, responseTaskCompletionSource);
            await RpcCall(operation, data);
            var result = readResponse ?await _taskMap[0].Task : null;
            return result;
        }

        private async Task RpcCall(Operation operation, byte[] data)
        {
            var stream = _client.GetStream();
            var outputStream = new MemoryStream();
            await outputStream.WriteAsync(new byte[] { (byte)operation, 0, 0, 0 });
            await outputStream.WriteAsync(new byte[] { 0, 0, 0, 0 });
            await outputStream.WriteAsync(BitConverter.GetBytes(data.Length));
            await outputStream.WriteAsync(data);
            await stream.WriteAsync(outputStream.ToArray());
        }
        
        private async Task<byte[]> ReadRpcResponse(CancellationToken token = default)
        {
            var stream = _client.GetStream();
            // read state number
            var buf = new byte[4];
            token.ThrowIfCancellationRequested();
            var state = await stream.ReadAsync(buf, token);
            token.ThrowIfCancellationRequested();
            if (state != buf.Length)
                throw new WrongProtocolException();
            // read content length
            buf = new byte[4];
            token.ThrowIfCancellationRequested();
            var n = await stream.ReadAsync(buf, token);
            token.ThrowIfCancellationRequested();
            if (n != buf.Length)
                throw new WrongProtocolException();
            var resLength = BitConverter.ToUInt32(buf);
            if (resLength == 0)
                return Array.Empty<byte>();
            // read content
            buf = new byte[resLength];
            n = await stream.ReadAsync(buf, token);
            token.ThrowIfCancellationRequested();
            if (n != buf.Length)
                throw new WrongProtocolException();
            return buf;
        }
    }
}