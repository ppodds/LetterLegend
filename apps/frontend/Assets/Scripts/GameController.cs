using System.Collections;
using System.Collections.Generic;
using UnityEngine;
public class GameController : MonoBehaviour
{

    public GameObject tile;
    public Block[] blockList = new Block[8];
    private Camera mainCamera;
    private void Awake()
    {
        ResetBlock();
    }


    void ResetBlock()
    {
        for (var i = 0; i < 8; i++)
        {
            Vector3 cameraPosition = Camera.main.transform.position;
            Vector3 bottomCenter = new Vector3(cameraPosition.x + i, cameraPosition.y - Camera.main.orthographicSize, 0f);
            Instantiate(blockList[i], bottomCenter, Quaternion.identity);
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
}
