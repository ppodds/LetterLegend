using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

public class Block : MonoBehaviour
{
    public float speed = 10f;
    private Camera mainCamera;

    private void Start()
    {
        mainCamera = Camera.main;
    }
    
    private void Update()
    {
        Vector3 mousePosition = Input.mousePosition;
        // mousePosition.z = transform.position.z - mainCamera.transform.position.z;
        Vector2 targetPosition = mainCamera.ScreenToWorldPoint(mousePosition);
        Vector2 currentPostion = transform.position;
        Vector2 unitVector = (targetPosition - currentPostion);
        unitVector = unitVector.normalized;
        if ((targetPosition - currentPostion).magnitude < speed * Time.deltaTime)
        {
            transform.position = targetPosition;
        }
        else
        {
            transform.position += (Vector3)unitVector * speed * Time.deltaTime;
        }
        // transform.position = Vector3.MoveTowards(transform.position, targetPosition, speed * Time.deltaTime);
    }
}
