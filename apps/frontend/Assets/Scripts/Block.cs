using TMPro;
using UnityEngine;

public class Block : MonoBehaviour
{
    private TextMeshPro _textMeshPro;
    private BoxCollider2D _testCollider;
    private Camera _mainCamera;

    private void Awake()
    {
        _textMeshPro = transform.Find("Square").transform.Find("Text").GetComponent<TextMeshPro>();
        _textMeshPro.text = "";
        _testCollider = gameObject.GetComponent<BoxCollider2D>();
        _mainCamera = Camera.main;
    }

    public string Contains(Vector2 position)
    {
        var worldPosition = _mainCamera.ScreenToWorldPoint(position);
        if (_testCollider.bounds.Contains((Vector2)worldPosition) && HandField.GetSelectBlockUI() != null
                                                                  && _textMeshPro.text == "")
        {
            _textMeshPro.text = HandField.GetSelectText();
            return "true";
        }
        else if (_testCollider.bounds.Contains((Vector2)worldPosition) && HandField.GetSelectBlockUI() == null
                                                                       && _textMeshPro.text != "")
        {
            string returnString = _textMeshPro.text;
            _textMeshPro.text = "";
            return returnString;
        }

        return "false";
    }
}