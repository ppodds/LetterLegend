using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Board : MonoBehaviour
{
    // Start is called before the first frame update
    public GameObject tile;
    void Start()
    {
        for (int i = 0; i < 26; i++)
        {
            for (int j = 0; j < 26; j++)
            {
                Instantiate(tile, new Vector3(-13+i, -13+j , 0), new Quaternion(), GameObject.Find("Board").transform);
            }
        }
        FindObjectOfType<MouseEventSystem>().MouseClickEvent.AddListener(OnMouseClick);
    }
    private void OnMouseClick(Vector2 position)
    {

    }
    // Update is called once per frame
    void Update()
    {
        
    }
}
