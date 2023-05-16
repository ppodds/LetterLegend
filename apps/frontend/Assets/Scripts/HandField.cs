using Unity.VisualScripting;
using UnityEngine;
using UnityEngine.UI;
using Random = UnityEngine.Random;

public class HandField : MonoBehaviour
{
    public GameObject blockUI;
    private GameObject[] _blockList;
    private Button _button;
    public GameObject handField;
    private static BlockUI _selectBlockUI;
    private Vector3 _selectBlockPosition;
    private MouseEventSystem _mouseEventSystem;

    private void Awake()
    {
        var currentPosition = handField.GetComponent<RectTransform>().position;
        _blockList = new GameObject[8];
        _button = transform.Find("Button").GetComponent<Button>();
        _button.transform.position = new Vector3(currentPosition.x + 165, currentPosition.y, 0f);
        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
        _mouseEventSystem = MouseEventSystem.GetInstance();
        for (var i = 0; i < _blockList.Length; i++)
        {
            var bottomCenter = new Vector3(currentPosition.x - 165 + 40 * i, currentPosition.y, 0f);
            _blockList[i] = Instantiate(blockUI, bottomCenter, Quaternion.identity, this.transform);
        }

        ResetBlock();
        _button.onClick.AddListener(ResetBlock);
        _mouseEventSystem.GetMouseClickedEvent().AddListener(MouseClicked);
        _mouseEventSystem.GetFirstClickedEvent().AddListener(FirstClicked);
        _mouseEventSystem.GetMouseDraggedEvent().AddListener(MouseDragged);
        Board.GetReleasedEvent().AddListener(MouseReleased);
        Board.GetRightClickedEvent().AddListener(AddBlock);
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

    private void MouseReleased(bool placed)
    {
        if (placed)
        {
            DeleteSelectObject();
        }
        else
        {
            if (_selectBlockUI != null) _selectBlockUI.transform.position = _selectBlockPosition;
        }

        _selectBlockUI = null;
        _selectBlockPosition = Vector3.zero;
    }

    public static BlockUI GetSelectBlockUI()
    {
        return _selectBlockUI;
    }

    public static string GetSelectText()
    {
        return _selectBlockUI.GetText();
    }

    private void DeleteSelectObject()
    {
        for (var i = 0; i < _blockList.Length; i++)
        {
            if (_blockList[i])
            {
                var block = _blockList[i].GetComponent<BlockUI>();
                if (block == _selectBlockUI)
                {
                    block.transform.SetParent(null);
                    Destroy(block);
                    _blockList[i] = null;
                    break;
                }
            }
        }
    }

    private void AddBlock(string text)
    {
        var currentPosition = handField.GetComponent<RectTransform>().position;
        for (var i = 0; i < _blockList.Length; i++)
        {
            if (_blockList[i] == null)
            {
                var bottomCenter = new Vector3(currentPosition.x - 165 + 40 * i, currentPosition.y, 0f);
                _blockList[i] = Instantiate(blockUI, bottomCenter, Quaternion.identity, this.transform);
                _blockList[i].GetComponent<BlockUI>().SetText(text);
                break;
            }
        }
    }
}