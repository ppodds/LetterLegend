using System;
using System.Collections.Generic;
using UnityEngine;

public class Board : MonoBehaviour
{
    public GameObject block;
    private readonly List<GameObject> _blocks = new List<GameObject>();
    private MouseEventSystem _mouseEventSystem;
    private HandField _handField;
    private Vector3 _boardMin;
    private Vector3 _boardMax;

    private void Awake()
    {
        var scale = block.transform.localScale.x;
        for (var i = 0; i < 26; i++)
        {
            for (var j = 0; j < 26; j++)
            {
                var tempBlock = Instantiate(block, new Vector3(i * scale - 17, j * scale - 17, 0f), Quaternion.identity,
                    GameObject.Find("Board").transform);
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
    }

    private async void MouseReleased(Vector2 position)
    {
        for (var i = 0; i < _blocks.Count; i++)
        {
            var tempBlock = _blocks[i];
            if (tempBlock.GetComponent<Block>().Contains(position) && _handField.GetSelectBlock())
            {
                var index = _handField.GetIndex();
                if (index == null)
                {
                    throw new Exception("HandField GetIndex failed");
                }

                uint x = (uint)i % 26;
                uint y = (uint)i / 26;
                var res = await GameManager.Instance.GameTcpClient.SetTile(x, y, index.Value);
                if (res)
                {
                    tempBlock.GetComponent<Block>().SetText(_handField.GetText());
                    _handField.DeleteSelectObject();
                    return;
                }
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
}