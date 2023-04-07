using System.Collections;
using System.Collections.Generic;
using UnityEngine;
public class GameController : MonoBehaviour
{

    public GameObject tile;
    public MouseEvent mouseEvent;
    
    private void Awake()
    {
        Instantiate(mouseEvent);
        ResetBlock();
    }


    void ResetBlock()
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
