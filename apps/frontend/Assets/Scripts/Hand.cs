using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Hand : MonoBehaviour
{
    public Block[] blockList = new Block[8];
    private void Awake()
    {
        for (var i = 0; i < 8; i++)
        {
            Vector3 cameraPosition = Camera.main.transform.position;
            Vector3 bottomCenter = new Vector3(cameraPosition.x -4 + i, cameraPosition.y - Camera.main.orthographicSize, 0f);
            Instantiate(blockList[i], bottomCenter, Quaternion.identity, this.transform);
        }
        FindObjectOfType<MouseEvent>().MouseClickEvent.AddListener(OnMouseClick);

    }
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }
    private void OnMouseClick(Vector2 position)
    {
        Debug.Log(position);
    }
}
