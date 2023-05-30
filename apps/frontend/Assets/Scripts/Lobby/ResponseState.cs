// using System;
// using System.Threading;
// using System.Threading.Tasks;
// using IO.Net;
// using Protos.Lobby;
// using Unity.VisualScripting.Antlr3.Runtime;
// using UnityEngine;
// using UnityEngine.UI;
// public class StateResponse : State
// {
//     private readonly RoomPanel _roomPanel;
//     public override async Task Handle()
//     {
//         await SetReadyButton();
//         Client.TransitionTo(new StateBroadcast());
//         Client.Handle();
//     }
//     
//     public StateResponse()
//     {
//         _roomPanel = GameObject.Find("RoomPanel").GetComponent<RoomPanel>();
//     }
//     
//     private async Task SetReadyButton()
//     {
//         var res = await SetReady();
//         if (res)
//         {
//             _roomPanel.transform.Find("ReadyBtn").GetComponent<Button>().image.color = Color.gray;
//             Debug.Log("set ready success");
//         }
//         else
//         {
//             Debug.Log("Set ready failed");
//         }
//     }
//     
//     private async Task<bool> SetReady()
//     {
//         var res = ReadyResponse.Parser.ParseFrom(await Client.Rpc(Operation.Ready));
//         if (!res.Success)
//         {
//             throw new Exception("Set Ready failed");
//         }
//         return true;
//     }
// }
