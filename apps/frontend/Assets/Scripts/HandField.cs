using UnityEngine;

public class HandField : MonoBehaviour{
    public TileUI[] tileList = new TileUI[8];
    public GameObject handField;
    private TileUI selectTileUI = null;
    private void Awake()
    {
        var currentPosition = handField.GetComponent<RectTransform>().position;
        for (var i = 0; i < tileList.Length; i++)
        {
            var bottomCenter = new Vector3(currentPosition.x - 180 + 40 * i, currentPosition.y, 0f);
            // cover the initial prefab reference
            tileList[i] = Instantiate(tileList[i], bottomCenter, Quaternion.identity, this.transform);
        }
        FindObjectOfType<MouseEventSystem>().mouseClickedEvent.AddListener(MouseClicked);
        FindObjectOfType<MouseEventSystem>().mouseReleasedEvent.AddListener(MouseReleased);
        FindObjectOfType<MouseEventSystem>().mouseDragEvent.AddListener(MouseDragged);
    }

    private void MouseClicked(Vector2 position)
    {
        for (var i = 0; i < tileList.Length; i++)
        {
            TileUI tileUI = tileList[i];
            if (tileUI.contain(position))
            {
                selectTileUI = tileUI;
                Debug.Log(i);
                break;
            }
        }
    }

    public void MouseDragged(Vector2 position)
    {
        if (selectTileUI != null)
        {
            selectTileUI.transform.position = position;
        }
    }
    private void MouseReleased(Vector2 position)
    {
        selectTileUI = null;
    }
}