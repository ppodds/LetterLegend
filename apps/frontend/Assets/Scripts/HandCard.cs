using System.Collections;
using System.Collections.Generic;
using Unity.VisualScripting;
using UnityEngine;

public class HandCard : MonoBehaviour
{
    public Transform uiParent;
    public Block[] blockList = new Block[8];
    private void Awake()
    {
        for (var i = 0; i < 8; i++)
        {
            Vector3 cameraPosition = Camera.main.transform.position;
            Vector3 bottomCenter = new Vector3(cameraPosition.x -8 + 2*i, cameraPosition.y - Camera.main.orthographicSize, 0f);
            // cover the initial prefab reference
            blockList[i] = Instantiate(blockList[i], bottomCenter, Quaternion.identity, this.transform);
            BoxCollider2D collider = blockList[i].GetComponent<BoxCollider2D>();
            // may change after
            collider.size = Vector2.one;
            collider.offset = Vector2.zero;
        }
        FindObjectOfType<MouseEventSystem>().MouseClickEvent.AddListener(OnMouseClick);
        FindObjectOfType<MouseEventSystem>().MouseUpEvent.AddListener(OnMouseUp);
    }

    void ResetBlock()
    {
        for (int i = 0; i < 8; i++)
        {
            blockList[i].NoClick();
            Vector3 cameraPosition = Camera.main.transform.position;
            Vector3 bottomCenter = new Vector3(cameraPosition.x -8 + 2*i, cameraPosition.y - Camera.main.orthographicSize, 0f);
            blockList[i].transform.position = bottomCenter;
        }
    }
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }

    private void OnMouseUp(Vector2 position)
    {
        ResetBlock();
    }
    private void OnMouseClick(Vector2 position)
    {
        Debug.Log(position);
        Vector2 worldClickPosition = Camera.main.ScreenToWorldPoint(position);
        Debug.Log(worldClickPosition);
        for (int i = 0; i < blockList.Length; i++)
        {
            Block block = blockList[i];
            if (block.GetComponent<BoxCollider2D>().bounds.Contains(worldClickPosition))
            {
                Debug.Log("Block clicked: " + i);
                blockList[i].IsClick();
                break;
            }
        }
    }
}
