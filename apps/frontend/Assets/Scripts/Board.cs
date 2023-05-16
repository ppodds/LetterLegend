using System.Collections.Generic;
using UnityEngine;

public class Board : MonoBehaviour
{
    public GameObject block;
    private List<GameObject> _blocks = new List<GameObject>();
    private MouseEventSystem _mouseEventSystem;
    private HandField _handField;

    private void Awake()
    {
        for (var i = 0; i < 26; i++)
        {
            for (var j = 0; j < 26; j++)
            {
                var tempBlock = Instantiate(block, new Vector3(i * 1.3f, j * 1.3f, 0f), Quaternion.identity,
                    GameObject.Find("Board").transform);
                _blocks.Add(tempBlock);
            }
        }

        _mouseEventSystem = MouseEventSystem.GetInstance();
        _mouseEventSystem.GetMouseReleasedEvent().AddListener(MouseReleased);
        _handField = HandField.GetInstance();
    }

    private void MouseReleased(Vector2 position)
    {
        foreach (var tempBlock in _blocks)
        {
            if (tempBlock.GetComponent<Block>().Contains(position) && _handField.GetSelectBlock())
            {
                tempBlock.GetComponent<Block>().SetText(_handField.GetText());
                _handField.DeleteSelectObject();
                return;
            }
        }

        _handField.ResetPosition();
    }
}