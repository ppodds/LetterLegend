using System;
using System.Collections.Generic;
using System.Linq;
using Protos.Game;
using UnityEngine;
using UnityEngine.SceneManagement;

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
    private Camera _camera;

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

        _boardMin = new Vector3(_blocks[0].transform.position.x - scale / 2,
            _blocks[0].transform.position.y - scale / 2, 0);
        _boardMax = new Vector3(_blocks[26 * 26 - 1].transform.position.x + scale / 2,
            _blocks[26 * 26 - 1].transform.position.y + scale / 2, 0);
        _camera = Camera.main;
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
                SceneManager.LoadScene(0);
                //GameManager.Instance.QuitGame();
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
                GameManager.Instance.SetPlayers(res.CurrentPlayer, res.NextPlayer);
                timer.ResetCurrentTime();
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
    }

    private async void MouseReleased(Vector2 position)
    {
        if (!_handField.GetSelectBlock() || _handField.GetIndex() == null
                                         || !Equals(GameManager.Instance.GetCurrentPlayer(),
                                             GameManager.Instance.GetMainPlayer()))
        {
            _handField.ResetPosition();
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

            await GameManager.Instance.GameTcpClient.SetTile(blockComponent.GetX(), blockComponent.GetY(),
                _handField.GetIndex().Value);
            blockComponent.SetText(_handField.GetText());
            _handField.DeleteSelectObject();
            return;
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
        var count = 0;
        Block targetBlock = null;
        for (var i = 0; i < board.Rows.Count; i++)
        {
            for (var j = 0; j < board.Rows[i].Columns.Count; j++)
            {
                var blockComponent = _blocks[i * board.Rows.Count + j].GetComponent<Block>();
                if (board.Rows[i].Columns[j].Tile != null)
                {
                    if (blockComponent.GetText() == "")
                    {
                        count++;
                        targetBlock = blockComponent;
                    }

                    blockComponent.SetText(board.Rows[i].Columns[j].Tile.Char);
                }
                else
                {
                    if (blockComponent.GetText() != "")
                    {
                        count++;
                        targetBlock = blockComponent;
                    }

                    blockComponent.SetText("");
                }
            }
        }

        if (count == 1)
        {
            StayFocus(targetBlock);
        }
    }

    private void StayFocus(Block target)
    {
        var targetPosition = target.transform.position;
        targetPosition.z = -10;
        var screenRef = targetPosition - _camera.transform.position;

        var screenBound = _camera.ScreenToWorldPoint(new Vector3(Screen.width, Screen.height, 0));
        if (screenBound.x + screenRef.x > _boardMax.x) targetPosition.x -= (screenBound.x + screenRef.x - _boardMax.x);
        if (screenBound.y + screenRef.y > _boardMax.y) targetPosition.y -= (screenBound.y + screenRef.y - _boardMax.y);

        var cameraMinCoordinate = _camera.ViewportToWorldPoint(new Vector3(0, 0, 0));
        if (cameraMinCoordinate.x + screenRef.x < _boardMin.x)
            targetPosition.x -= (cameraMinCoordinate.x + screenRef.x - _boardMin.x);
        if (cameraMinCoordinate.y + screenRef.y < _boardMin.y)
            targetPosition.y -= (cameraMinCoordinate.y + screenRef.y - _boardMin.y);

        _camera.transform.position = targetPosition;
    }
}