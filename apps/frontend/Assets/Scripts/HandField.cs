using UnityEngine;

public class HandField : MonoBehaviour{
    public TileUI[] tileList = new TileUI[8];
    public GameObject handField;
    private TileUI _selectTileUI;
    private MouseEventSystem _mouseEventSystem;
    
    private void Awake()
    {
        _selectTileUI = null;
        _mouseEventSystem = MouseEventSystem.GetInstance();
        var currentPosition = handField.GetComponent<RectTransform>().position;
        for (var i = 0; i < tileList.Length; i++)
        {
            var bottomCenter = new Vector3(currentPosition.x - 165 + 40 * i, currentPosition.y, 0f);
            tileList[i] = Instantiate(tileList[i], bottomCenter, Quaternion.identity, this.transform);
        }
        _mouseEventSystem.GetMouseClickedEvent().AddListener(MouseClicked);
        _mouseEventSystem.GetFirstClickedEvent().AddListener(FirstClicked);
        _mouseEventSystem.GetMouseReleasedEvent().AddListener(MouseReleased);
        _mouseEventSystem.GetMouseDraggedEvent().AddListener(MouseDragged);
    }

    private void MouseClicked(Vector2 position)
    {
        _selectTileUI = null;
        
    }

    private void FirstClicked(Vector2 position)
    {
        for (var i = 0; i < tileList.Length; i++)
        {
            var tileUI = tileList[i];
            if (!tileUI.Contains(position)) continue;
            _selectTileUI = tileUI;
            Debug.Log(i);
            break;
        }
    }

    private void MouseDragged(Vector2 position)
    {
        if (_selectTileUI != null)
        {
            _selectTileUI.transform.position = position;
        }
    }
    
    private void MouseReleased(Vector2 position)
    {
        _selectTileUI = null;
    }
}