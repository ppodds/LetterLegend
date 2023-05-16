using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

public class Board : MonoBehaviour
{
    public GameObject block;
    private List<GameObject> _blocks = new List<GameObject>();
    private MouseEventSystem _mouseEventSystem;
    private static UnityEvent<bool> _releasedEvent;
    private static UnityEvent<string> _rightClickedEvent;

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
        _mouseEventSystem.GetMouseRightClickedEvent().AddListener(MouseRightClicked);
        _releasedEvent = new UnityEvent<bool>();
        _rightClickedEvent = new UnityEvent<string>();
    }

    private void MouseReleased(Vector2 position)
    {
        foreach (var tempBlock in _blocks)
        {
            if (tempBlock.GetComponent<Block>().Contains(position) == "true")
            {
                _releasedEvent.Invoke(true);
                return;
            }
        }

        _releasedEvent.Invoke(false);
    }

    private void MouseRightClicked(Vector2 position)
    {
        foreach (var tempBlock in _blocks)
        {
            string returnString = tempBlock.GetComponent<Block>().Contains(position);
            if (returnString != "true" && returnString != "false")
            {
                _rightClickedEvent.Invoke(returnString);
                return;
            }
        }
    }

    public static UnityEvent<bool> GetReleasedEvent()
    {
        return _releasedEvent;
    }

    public static UnityEvent<string> GetRightClickedEvent()
    {
        return _rightClickedEvent;
    }
}