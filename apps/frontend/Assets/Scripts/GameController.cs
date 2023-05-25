using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Serialization;

public class GameController : MonoBehaviour
{
    public GameObject tile;
    [FormerlySerializedAs("mouseEvent")] public MouseEventSystem mouseEventSystem;

    private void Awake()
    {
    }

    // Start is called before the first frame update
    void Start()
    {
    }

    // Update is called once per frame
    void Update()
    {
    }
}