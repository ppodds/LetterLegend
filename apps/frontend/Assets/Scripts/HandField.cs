using System;
using System.Collections.Generic;
using Protos.Game;
using UnityEngine;

public class HandField : MonoBehaviour
{
    private static HandField _handField;
    public GameObject blockUI;
    private GameObject[] _blockList;
    public GameObject handField;
    private BlockUI _selectBlockUI;
    private Vector3 _selectBlockPosition;
    private MouseEventSystem _mouseEventSystem;

    private void Awake()
    {
        if (_handField != null && _handField != this)
        {
            Destroy(gameObject);
            return;
        }

        _handField = this;
        var currentPosition = handField.GetComponent<RectTransform>().position;
        var widthRef = (handField.GetComponent<RectTransform>().rect.width -
                        blockUI.GetComponent<RectTransform>().rect.width) / 2;
        _blockList = new GameObject[8];
        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
        _mouseEventSystem = MouseEventSystem.GetInstance();
        for (var i = 0; i < _blockList.Length; i++)
        {
            var bottomCenter =
                new Vector3(currentPosition.x - widthRef + (blockUI.GetComponent<RectTransform>().rect.width + 10) * i,
                    currentPosition.y, 0f);
            _blockList[i] = Instantiate(blockUI, bottomCenter, Quaternion.identity, this.transform);
        }

        SetHandField(GameManager.Instance.GetHandCards());
        _mouseEventSystem.GetMouseClickedEvent().AddListener(MouseClicked);
        _mouseEventSystem.GetFirstClickedEvent().AddListener(FirstClicked);
        _mouseEventSystem.GetMouseDraggedEvent().AddListener(MouseDragged);
    }

    public static HandField GetInstance()
    {
        return _handField;
    }

    private async void ResetBlock()
    {
        if (GetCount() < 8)
        {
            return;
        }

        var res = await GameManager.Instance.GameTcpClient.GetNewCard();
        SetHandField(res);
    }

    private void MouseClicked(Vector2 position)
    {
        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
    }

    private void FirstClicked(Vector2 position)
    {
        foreach (var tempBlock in _blockList)
        {
            if (!tempBlock)
            {
                continue;
            }

            var block = tempBlock.GetComponent<BlockUI>();
            if (!block || !block.Contains(position))
            {
                continue;
            }

            _selectBlockUI = block;
            _selectBlockPosition = _selectBlockUI.transform.position;
            break;
        }
    }

    private void MouseDragged(Vector2 position)
    {
        if (_selectBlockUI != null)
        {
            _selectBlockUI.transform.position = position;
        }
    }

    public void ResetPosition()
    {
        if (!_selectBlockUI)
        {
            return;
        }

        _selectBlockUI.transform.position = _selectBlockPosition;
        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
    }

    public bool GetSelectBlock()
    {
        return _selectBlockUI != null;
    }

    public uint? GetIndex()
    {
        if (_selectBlockUI == null)
        {
            return null;
        }

        for (var i = 0; i < _blockList.Length; i++)
        {
            if (_blockList[i] && _blockList[i].GetComponent<BlockUI>() == _selectBlockUI) return (uint)i;
        }

        return null;
    }

    public void DeleteSelectObject()
    {
        for (var i = 0; i < _blockList.Length; i++)
        {
            if (!_blockList[i])
            {
                continue;
            }

            var block = _blockList[i].GetComponent<BlockUI>();
            if (block != _selectBlockUI)
            {
                continue;
            }

            Destroy(block.gameObject);
            _blockList[i] = null;
            break;
        }

        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
    }

    public string GetText()
    {
        return _selectBlockUI != null ? _selectBlockUI.GetText() : null;
    }

    public void SetHandField(List<HandCard> handCards)
    {
        for (var i = 0; i < handCards.Count; i++)
        {
            if (handCards[i].Card == null)
            {
                continue;
            }

            if (_blockList[i])
            {
                _blockList[i].GetComponent<BlockUI>().SetText(handCards[i].Card.Symbol);
                continue;
            }

            var currentPosition = handField.GetComponent<RectTransform>().position;
            var widthRef = (handField.GetComponent<RectTransform>().rect.width -
                            blockUI.GetComponent<RectTransform>().rect.width) / 2;
            var bottomCenter =
                new Vector3(currentPosition.x - widthRef + (blockUI.GetComponent<RectTransform>().rect.width + 10) * i,
                    currentPosition.y, 0f);
            _blockList[i] = Instantiate(blockUI, bottomCenter, Quaternion.identity, this.transform);
            _blockList[i].GetComponent<BlockUI>().SetText(handCards[i].Card.Symbol);
        }
    }

    private int GetCount()
    {
        var count = 0;
        foreach (var tempBlock in _blockList)
        {
            if (tempBlock)
            {
                count++;
            }
        }

        return count;
    }
}