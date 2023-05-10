using UnityEngine;

public class TileUI : MonoBehaviour
{
    private BoxCollider2D _testCollider;

    private void Awake()
    {
        _testCollider = gameObject.AddComponent<BoxCollider2D>();
        _testCollider.size = new Vector2(30,30);
    }

    public bool Contains(Vector2 position)
    {
        return _testCollider.bounds.Contains(position) ? true : false;
    }
}