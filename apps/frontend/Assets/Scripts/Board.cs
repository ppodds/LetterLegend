using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Board : MonoBehaviour
{
    // Start is called before the first frame update
    public GameObject tile;
    private MouseEventSystem mouseEventSystem;
    public void Start()
    {
        mouseEventSystem = MouseEventSystem.GetInstance();
        for (int i = 0; i < 26; i++)
        {
            for (int j = 0; j < 26; j++)
            {
                Instantiate(tile, new Vector3(-13+i, -13+j , 0), new Quaternion(), GameObject.Find("Board").transform);
            }
        }
        mouseEventSystem.GetMouseClickedEvent().AddListener(OnMouseClick);
    }
    private void OnMouseClick(Vector2 position)
    {

    }
    // Update is called once per frame
    public void Update()
    {
        
    }
}
