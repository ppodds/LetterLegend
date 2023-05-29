using UnityEngine;

public class EdgeScrolling : MonoBehaviour
{
    private Camera _camera;
    private float _scrollSpeed;
    private float _scrollZone;
    private Vector3 _screenBound;

    private void Awake()
    {
        _camera = Camera.main;
        _scrollSpeed = 15f;
        _scrollZone = 20f;
    }

    private void Update()
    {
        var mousePosition = Input.mousePosition;
        _screenBound = new Vector3(Screen.width, Screen.height, _camera.transform.position.z);
        if (OnEdge(mousePosition, _screenBound.x, _screenBound.y))
        {
            var mouseDirection = mousePosition - new Vector3(_screenBound.x / 2f, _screenBound.y / 2f, 0f);
            mouseDirection.Normalize();
            var scrollRef = _scrollSpeed * Time.deltaTime;
            EdgeHandle(mouseDirection * scrollRef);
        }
    }

    private bool OnEdge(Vector3 mousePosition, float screenWidth, float screenHeight)
    {
        return mousePosition.x < _scrollZone ||
               mousePosition.x > screenWidth - _scrollZone ||
               mousePosition.y < _scrollZone ||
               mousePosition.y > screenHeight - _scrollZone;
    }

    private void EdgeHandle(Vector3 scrollRef)
    {
        var worldPosition = _camera.ScreenToWorldPoint(_camera.transform.position + scrollRef);
        var worldScreenPosition = _camera.ScreenToWorldPoint(_screenBound + scrollRef);
        if ((worldPosition.x < Board.BoardMin.x && scrollRef.x < 0)
            || (worldScreenPosition.x > Board.BoardMax.x && scrollRef.x > 0)) scrollRef.x = 0;
        if ((worldPosition.y < Board.BoardMin.y && scrollRef.y < 0)
            || (worldScreenPosition.y > Board.BoardMax.y && scrollRef.y > 0)) scrollRef.y = 0;
        _camera.transform.position += scrollRef;
    }
}