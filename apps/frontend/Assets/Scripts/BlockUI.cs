using TMPro;
using UnityEngine;

public class BlockUI : MonoBehaviour
{
    private TextMeshProUGUI _textMeshProUGUI;
    private BoxCollider2D _testCollider;
    
    private void Awake()
    {
        _textMeshProUGUI = transform.Find("Text").GetComponent<TextMeshProUGUI>();
        _textMeshProUGUI.text = "";
        _testCollider = gameObject.GetComponent<BoxCollider2D>();
        _testCollider.size = new Vector2(30,30);
    }
    
    public bool Contains(Vector2 position)
    {
        return _testCollider.bounds.Contains(position) ? true : false;
    }

    public void SetText(string text)
    {
        _textMeshProUGUI.text = text;
    }

    public string GetText()
    {
        return _textMeshProUGUI.text;
    }
}