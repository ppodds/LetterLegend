// using System;
// using System.Threading;
// using System.Threading.Tasks;
// using Protos.Lobby;
// using UnityEngine;
//
// public class StateBroadcast : State
// {
//     private readonly RoomPanel _roomPanel;
//     private readonly CancellationTokenSource _cancellationTokenSource;
//     public override async Task Handle()
//     {
//         CancellationToken token = _cancellationTokenSource.Token;
//         try
//         {
//             await WaitLoop(token);
//         }
//         catch (OperationCanceledException e)
//         {
//             Debug.Log(e);
//             Client.TransitionTo(new StateResponse());
//             Client.Handle();
//         }
//     }
//
//     public void StopLoop()
//     {
//         Debug.Log(_cancellationTokenSource.IsCancellationRequested);
//         _cancellationTokenSource.Cancel();
//         Debug.Log(_cancellationTokenSource.IsCancellationRequested);
//     }
//     
//     public StateBroadcast()
//     {
//         _cancellationTokenSource = new CancellationTokenSource();
//         _roomPanel = GameObject.Find("RoomPanel").GetComponent<RoomPanel>();
//     }
//
//     private async Task WaitLoop(CancellationToken cancellationToken)
//     {
//         var inLobby = true;
//         while (true)
//         {
//             if (cancellationToken.IsCancellationRequested)
//             {
//                 Debug.Log("terminate the loop");
//                 throw new OperationCanceledException(cancellationToken);  
//             }
//             var res = await WaitLobbyBroadcast(cancellationToken);
//             switch (res.Event)
//             {
//                 case LobbyEvent.Join:
//                     _roomPanel.ClearList();
//                     _roomPanel.Lobby = res.Lobby;
//                     _roomPanel.UpdateRoom();
//                     break;
//                 case LobbyEvent.Leave:
//                     _roomPanel.ClearList();
//                     _roomPanel.Lobby = res.Lobby;
//                     _roomPanel.UpdateRoom();
//                     break;
//                 case LobbyEvent.Destroy:
//                     _roomPanel.lobbyPanel.SetActive(true);
//                     _roomPanel.gameObject.SetActive(false);
//                     inLobby = false;
//                     break;
//                 case LobbyEvent.Start:
//                     GameManager.Instance.StartGame();
//                     //TODO switch to InGame State
//                     inLobby = false;
//                     break;
//                 default:
//                     throw new ArgumentOutOfRangeException();
//             }
//         }
//     }
//     
//     private async Task<LobbyBroadcast> WaitLobbyBroadcast(CancellationToken cancellationToken = default)
//     {
//         // if (cancellationToken.IsCancellationRequested)
//         // {
//         //     Debug.Log("terminate the loop");
//         //     throw new OperationCanceledException(cancellationToken);  
//         // }
//         var res = await Client.ReadBroadcast(cancellationToken);
//         return LobbyBroadcast.Parser.ParseFrom(res);
//     }
// }
