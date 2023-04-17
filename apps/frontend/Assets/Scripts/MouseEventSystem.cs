using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.EventSystems;
public class MouseClickEvent : UnityEvent<Vector2>
{
  
}


public class MouseDragEvent : UnityEvent<Vector2>
{
  
}

public class MouseUpEvent : UnityEvent<Vector2>
{
    
}
public class MouseEventSystem : MonoBehaviour
{
    public MouseClickEvent MouseClickEvent = new MouseClickEvent();
    public MouseUpEvent MouseUpEvent = new MouseUpEvent();
    public MouseDragEvent MouseDragEvent;

    private void Update()
    {
        if (Input.GetMouseButtonDown(0))
        {
            MouseClickEvent.Invoke(Input.mousePosition);
        }
        else if (Input.GetMouseButtonUp(0))
        {
            MouseUpEvent.Invoke(Input.mousePosition);
        }
    }

    private void Start()
    {

    }

    
}
