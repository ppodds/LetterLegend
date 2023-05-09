using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Rendering;

public class TileUI : MonoBehaviour
{
    // Start is called before the first frame update
    public bool contain(Vector2 position)
    {
        BoxCollider2D collider2D = GetComponent<BoxCollider2D>();
        collider2D.size = new Vector2(30,30);
        if (collider2D.bounds.Contains(position)) return true;
        return false;
    }
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
