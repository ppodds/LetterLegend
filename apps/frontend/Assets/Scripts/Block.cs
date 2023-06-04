using TMPro;
using UnityEngine;

public class Block : MonoBehaviour
{
    private TextMeshPro _textMeshPro;
    private BoxCollider2D _testCollider;
    private Camera _mainCamera;
    private MouseEventSystem _mouseEventSystem;
    private HandField _handField;
    private uint _x;
    private uint _y;

    private void Awake()
    {
        _textMeshPro = transform.Find("Square").transform.Find("Text").GetComponent<TextMeshPro>();
        _textMeshPro.text = "";
        _testCollider = gameObject.GetComponent<BoxCollider2D>();
        _mainCamera = Camera.main;
        _mouseEventSystem = MouseEventSystem.GetInstance();
        _mouseEventSystem.GetMouseRightClickedEvent().AddListener(MouseRightClicked);
        _handField = HandField.GetInstance();
    }

    public bool Contains(Vector2 position)
    {
        var worldPosition = _mainCamera.ScreenToWorldPoint(position);
        return _testCollider.bounds.Contains((Vector2)worldPosition);
    }

    private async void MouseRightClicked(Vector2 position)
    {
        if (!Contains(position) || _textMeshPro.text == "")
        {
            return;
        }
        
        var res = await GameManager.Instance.GameTcpClient.Cancel(_x, _y);
        _handField.AddBlock(res);
        _textMeshPro.text = "";
    }

    public void SetText(string text)
    {
        _textMeshPro.text = text;
    }

    public string GetText()
    {
        return _textMeshPro.text;
    }

    public void SetX(int x)
    {
        _x = (uint)x;
    }

    public uint GetX()
    {
        return _x;
    }

    public void SetY(int y)
    {
        _y = (uint)y;
    }

    public uint GetY()
    {
        return _y;
    }
}