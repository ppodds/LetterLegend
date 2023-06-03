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
        private readonly Dictionary<uint, TaskCompletionSource<byte[]>> _taskMap;
        private readonly System.Random _random;
        private Task _receiveLoop;
        private readonly CancellationTokenSource _cancellationTokenSource;
        public RoomPanel RoomPanel { get; set; }
        public Board Board { get; set; }

        enum Broadcast
        {
            Lobby = 0,
            Game = 1
        }

        public GameTcpClient(string host, int port)
        {
            _host = host;
            _port = port;
            _client = new TcpClient();
            _taskMap = new Dictionary<uint, TaskCompletionSource<byte[]>>();
            _random = new System.Random();
            _cancellationTokenSource = new CancellationTokenSource();
        }

        public bool IsConnected()
        {
            return _client.Connected;
        }

        private void Loop()
        {
            var token = _cancellationTokenSource.Token;
            _receiveLoop = Task.Run(async () =>
            {
                var stream = _client.GetStream();
                while (true)
                {
                    try
                    {
                        if (token.IsCancellationRequested)
                            token.ThrowIfCancellationRequested();
                        // read state
                        var buf = new byte[4];
                        var n = await stream.ReadAsync(buf);
                        if (n != buf.Length)
                            throw new WrongProtocolException();
                        var state = BitConverter.ToUInt32(buf);
                        // read length
                        buf = new byte[4];
                        n = await stream.ReadAsync(buf);
                        if (n != buf.Length)
                            throw new WrongProtocolException();
                        var resLength = BitConverter.ToUInt32(buf);
                        if (resLength == 0)
                            _taskMap[state].SetResult(Array.Empty<byte>());
                        // read data
                        buf = new byte[resLength];
                        n = await stream.ReadAsync(buf);
                        if (n != buf.Length)
                            throw new WrongProtocolException();
                        if (state == (uint)(Broadcast.Lobby))
                        {
                            var lobbyRes = LobbyBroadcast.Parser.ParseFrom(buf);
                            RoomPanel.BroadcastEnqueue(lobbyRes);
                        }
                        else if (state == (uint)(Broadcast.Game))
                        {
                            var gameRes = GameBroadcast.Parser.ParseFrom(buf);
                            // TODO: send message to board main thread
                            // Board.SetGameState(gameRes);
                        }
                        else if (_taskMap.ContainsKey(state))
                        {
                            _taskMap[state].SetResult(buf);
                        }
                    }
                    catch (Exception ex)
                    {
                        Debug.LogException(ex);
                        break;
                    }
                }
            }, token);
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

            var res = ConnectResponse.Parser.ParseFrom(await Rpc(Operation.Connect, stream.ToArray()));
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

        public async Task<Protos.Game.Board> StartGame()
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

            _cancellationTokenSource.Cancel();
            _client.Close();
        }

        private async Task<byte[]> Rpc(Operation operation, bool readResponse = true)
        {
            return await Rpc(operation, Array.Empty<byte>(), readResponse);
        }

        private async Task<byte[]> Rpc(Operation operation, byte[] data, bool readResponse = true)
        {
            uint thirtyBits = (uint)_random.Next(2, 1 << 30);
            uint twoBits = (uint)_random.Next(1 << 2);
            uint state = (thirtyBits << 2) | twoBits;
            var responseTaskCompletionSource = new TaskCompletionSource<byte[]>();
            _taskMap.Add(state, responseTaskCompletionSource);
            await RpcCall(operation, data, state);
            var result = readResponse ? await _taskMap[state].Task : null;
            _taskMap.Remove(state);
            return result;
        }

        private async Task RpcCall(Operation operation, byte[] data, uint state)
        {
            var stream = _client.GetStream();
            var t = BitConverter.GetBytes(state);
            if (BitConverter.IsLittleEndian)
                Array.Reverse(t);
            var outputStream = new MemoryStream();
            await outputStream.WriteAsync(new byte[] { (byte)operation, 0, 0, 0 });
            await outputStream.WriteAsync(t);
            await outputStream.WriteAsync(BitConverter.GetBytes(data.Length));
            await outputStream.WriteAsync(data);
            await stream.WriteAsync(outputStream.ToArray());
        }
    }
}