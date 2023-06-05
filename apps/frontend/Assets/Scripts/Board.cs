using System;
using System.Collections.Generic;
using System.Linq;
using Protos.Game;
using UnityEngine;

public class Board : MonoBehaviour
{
    public Timer timer;
    public PlayerShowText playerShowText;
    public GameObject block;
    private readonly List<GameObject> _blocks = new List<GameObject>();
    private MouseEventSystem _mouseEventSystem;
    private HandField _handField;
    private Vector3 _boardMin;
    private Vector3 _boardMax;
    private Queue<GameBroadcast> _gameBroadcasts;

    private void Awake()
    {
        GameManager.Instance.GameTcpClient.Board = this;
        var scale = block.transform.localScale.x;
        for (var i = 0; i < 26; i++)
        {
            for (var j = 0; j < 26; j++)
            {
                var tempBlock = Instantiate(block, new Vector3(i * scale - 17, j * scale - 17, 0f), Quaternion.identity,
                    GameObject.Find("Board").transform);
                var blockScript = tempBlock.GetComponent<Block>();
                blockScript.SetX(i);
                blockScript.SetY(j);
                _blocks.Add(tempBlock);
            }
        }

        _boardMin = new Vector3(_blocks[0].transform.position.x - scale,
            _blocks[0].transform.position.y - scale, 0);
        _boardMax = new Vector3(_blocks[26 * 26 - 1].transform.position.x + scale,
            _blocks[26 * 26 - 1].transform.position.y + scale, 0);
        _mouseEventSystem = MouseEventSystem.GetInstance();
        _mouseEventSystem.GetMouseReleasedEvent().AddListener(MouseReleased);
        _handField = HandField.GetInstance();
        _gameBroadcasts = new Queue<GameBroadcast>();
    }

    public void Update()
    {
        GameBroadcast res;
        lock (_gameBroadcasts)
        {
            if (_gameBroadcasts.Count == 0)
            {
                return;
            }

            res = _gameBroadcasts.Dequeue();
        }

        switch (res.Event)
        {
            case GameEvent.Destroy:
                //滾回房間
                break;
            case GameEvent.Leave:
                //Debug.Log(res.Players);
                break;
            case GameEvent.Shuffle:
                break;
            case GameEvent.PlaceTile:
                SetBoard(res.Board);
                break;
            case GameEvent.FinishTurn:
                playerShowText.SetPlayerName(res.CurrentPlayer, res.NextPlayer);
                timer.ResetCurrentTime();
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
    }

    private async void MouseReleased(Vector2 position)
    {
        if (!_handField.GetSelectBlock() || _handField.GetIndex() == null)
        {
            return;
        }

        foreach (var tempBlock in _blocks)
        {
            var blockComponent = tempBlock.GetComponent<Block>();
            if (!blockComponent.Contains(position)
                || blockComponent.GetText() != "")
            {
                continue;
            }
            try
            {
                await GameManager.Instance.GameTcpClient.SetTile(blockComponent.GetX(), blockComponent.GetY(), 
                    _handField.GetIndex().Value);
                blockComponent.SetText(_handField.GetText());
                _handField.DeleteSelectObject();
                return;
            }
            catch (Exception ex)
            {
                _handField.ResetPosition();
                return;
            }
        }

        _handField.ResetPosition();
    }

    public Vector3 GetBoardMin()
    {
        return _boardMin;
    }

    public Vector3 GetBoardMax()
    {
        return _boardMax;
    }

    public void BroadcastEnqueue(GameBroadcast gameBroadcast)
    {
        lock (_gameBroadcasts)
        {
            _gameBroadcasts.Enqueue(gameBroadcast);
        }
    }

    private void SetBoard(Protos.Game.Board board)
    {
        for (var i = 0; i < board.Rows.Count; i++)
        {
            for (var j = 0; j < board.Rows[i].Columns.Count; j++)
            {
                if (board.Rows[i].Columns[j].Tile != null)
                {
                    _blocks[i * board.Rows.Count + j].GetComponent<Block>().SetText(board.Rows[i].Columns[j].Tile.Char);
                }
                else
                {
                    _blocks[i * board.Rows.Count + j].GetComponent<Block>().SetText("");
                }
            }
        }
    }
}