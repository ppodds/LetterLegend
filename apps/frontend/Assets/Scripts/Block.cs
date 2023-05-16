using TMPro;
using UnityEngine;

public class Block : MonoBehaviour
{
    private TextMeshPro _textMeshPro;
    private BoxCollider2D _testCollider;
    private Camera _mainCamera;
    private MouseEventSystem _mouseEventSystem;
    private HandField _handField;

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
        if (_testCollider.bounds.Contains((Vector2)worldPosition) && _textMeshPro.text == "")
        {
            return true;
        }

        return false;
    }

    private void MouseRightClicked(Vector2 position)
    {
        var worldPosition = _mainCamera.ScreenToWorldPoint(position);
        if (_testCollider.bounds.Contains((Vector2)worldPosition) && _textMeshPro.text != "")
        {
            _handField.AddBlock(_textMeshPro.text);
            _textMeshPro.text = "";
        }
    }

    public void SetText(string text)
    {
        _textMeshPro.text = text;
    }
}