using UnityEngine;
using Random = UnityEngine.Random;

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
        var widthRef = (handField.GetComponent<RectTransform>().rect.width - blockUI.GetComponent<RectTransform>().rect.width) / 2;
        _blockList = new GameObject[8];
        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
        _mouseEventSystem = MouseEventSystem.GetInstance();
        for (var i = 0; i < _blockList.Length; i++)
        {
            var bottomCenter = new Vector3(currentPosition.x - widthRef + (blockUI.GetComponent<RectTransform>().rect.width + 10) * i, currentPosition.y, 0f);
            _blockList[i] = Instantiate(blockUI, bottomCenter, Quaternion.identity, this.transform);
        }

        ResetBlock();
        _mouseEventSystem.GetMouseClickedEvent().AddListener(MouseClicked);
        _mouseEventSystem.GetFirstClickedEvent().AddListener(FirstClicked);
        _mouseEventSystem.GetMouseDraggedEvent().AddListener(MouseDragged);
    }

    public static HandField GetInstance()
    {
        return _handField;
    }

    private void ResetBlock()
    {
        for (var i = 0; i < _blockList.Length; i++)
        {
            if (_blockList[i])
            {
                var block = _blockList[i].GetComponent<BlockUI>();
                if (block) block.SetText(char.ToString((char)(Random.Range(0, 26) + 'A')));
            }
        }
    }

    private void MouseClicked(Vector2 position)
    {
        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
    }

    private void FirstClicked(Vector2 position)
    {
        for (var i = 0; i < _blockList.Length; i++)
        {
            if (_blockList[i])
            {
                var block = _blockList[i].GetComponent<BlockUI>();
                if (!block || !block.Contains(position)) continue;
                _selectBlockUI = block;
                _selectBlockPosition = _selectBlockUI.transform.position;
                break;
            }
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
        _selectBlockUI.transform.position = _selectBlockPosition;
        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
    }

    public bool GetSelectBlock()
    {
        if (_selectBlockUI != null) return true;
        return false;
    }

    public void DeleteSelectObject()
    {
        for (var i = 0; i < _blockList.Length; i++)
        {
            if (_blockList[i])
            {
                var block = _blockList[i].GetComponent<BlockUI>();
                if (block == _selectBlockUI)
                {
                    Destroy(block.gameObject);
                    _blockList[i] = null;
                    break;
                }
            }
        }

        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
    }

    public string GetText()
    {
        if (GetSelectBlock()) return _selectBlockUI.GetText();
        return null;
    }

    public void AddBlock(string text)
    {
        var currentPosition = handField.GetComponent<RectTransform>().position;
        var widthRef = (handField.GetComponent<RectTransform>().rect.width - blockUI.GetComponent<RectTransform>().rect.width) / 2;
        for (var i = 0; i < _blockList.Length; i++)
        {
            if (_blockList[i] == null)
            {
                var bottomCenter = new Vector3(currentPosition.x - widthRef + (blockUI.GetComponent<RectTransform>().rect.width + 10) * i, currentPosition.y, 0f);
                _blockList[i] = Instantiate(blockUI, bottomCenter, Quaternion.identity, this.transform);
                _blockList[i].GetComponent<BlockUI>().SetText(text);
                break;
            }
        }
    }
}