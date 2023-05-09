using UnityEngine;
using System;

public class MouseEventSystem : MonoBehaviour
{
    public MouseClickedEvent mouseClickedEvent = new MouseClickedEvent();
    public MouseReleasedEvent mouseReleasedEvent = new MouseReleasedEvent();
    public MouseDragEvent mouseDragEvent = new MouseDragEvent();
    private DateTime? eventStart;

    private void Update()
    {
        if (Input.GetMouseButton(0))
        {
            if (eventStart == null)
            {
                eventStart = DateTime.Now;
            }
            else if (DateTime.Now.Subtract((DateTime) eventStart).TotalMilliseconds > 150)
            {
                mouseDragEvent.Invoke(Input.mousePosition);
            }
        }
        else if (Input.GetMouseButtonUp(0) && eventStart != null)
        {
            if (DateTime.Now.Subtract((DateTime) eventStart).TotalMilliseconds > 150)
            {
                mouseReleasedEvent.Invoke(Input.mousePosition);
            }
            else
            {
                mouseClickedEvent.Invoke(Input.mousePosition);
            }
            eventStart = null;
        }
    }

    private void Start()
    {

    }
}
