using UnityEngine;
using System;

public class MouseEventSystem : MonoBehaviour
{
    private static MouseEventSystem _mouseEventSystem;
    private MouseClickedEvent _mouseClickedEvent;
    private FirstClickedEvent _firstClickedEvent;
    private MouseReleasedEvent _mouseReleasedEvent;
    private MouseDraggedEvent _mouseDraggedEvent;
    private MouseRightClickedEvent _mouseRightClickedEvent;
    private DateTime? _eventStart;

    public static MouseEventSystem GetInstance()
    {
        return _mouseEventSystem;
    }

    public MouseClickedEvent GetMouseClickedEvent()
    {
        return _mouseClickedEvent;
    }

    public FirstClickedEvent GetFirstClickedEvent()
    {
        return _firstClickedEvent;
    }

    public MouseDraggedEvent GetMouseDraggedEvent()
    {
        return _mouseDraggedEvent;
    }

    public MouseReleasedEvent GetMouseReleasedEvent()
    {
        return _mouseReleasedEvent;
    }

    public MouseRightClickedEvent GetMouseRightClickedEvent()
    {
        return _mouseRightClickedEvent;
    }

    private void Awake()
    {
        if (_mouseEventSystem != null && _mouseEventSystem != this)
        {
            Destroy(gameObject);
            return;
        }

        _mouseEventSystem = this;
        DontDestroyOnLoad(this.gameObject);
        _mouseClickedEvent = new MouseClickedEvent();
        _firstClickedEvent = new FirstClickedEvent();
        _mouseReleasedEvent = new MouseReleasedEvent();
        _mouseDraggedEvent = new MouseDraggedEvent();
        _mouseRightClickedEvent = new MouseRightClickedEvent();
        _eventStart = null;
    }

    private void Update()
    {
        if (Input.GetMouseButton(0))
        {
            if (_eventStart == null)
            {
                _firstClickedEvent.Invoke(Input.mousePosition);
                _eventStart = DateTime.Now;
            }
            else if (DateTime.Now.Subtract((DateTime)_eventStart).TotalMilliseconds > 150)
            {
                _mouseDraggedEvent.Invoke(Input.mousePosition);
            }
        }
        else if (Input.GetMouseButtonDown(1))
        {
            _eventStart = DateTime.Now;
        }
        else if (Input.GetMouseButtonUp(0) && _eventStart != null)
        {
            if (DateTime.Now.Subtract((DateTime)_eventStart).TotalMilliseconds > 150)
            {
                _mouseReleasedEvent.Invoke(Input.mousePosition);
            }
            else
            {
                _mouseClickedEvent.Invoke(Input.mousePosition);
            }

            _eventStart = null;
        }
        else if (Input.GetMouseButtonUp(1) && _eventStart != null)
        {
            if (DateTime.Now.Subtract((DateTime)_eventStart).TotalMilliseconds <= 150)
            {
                _mouseRightClickedEvent.Invoke(Input.mousePosition);
            }

            _eventStart = null;
        }
    }
}