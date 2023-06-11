using System.Collections.Generic;
using TMPro;
using UnityEngine;

public class Dict : MonoBehaviour
{
    private List<string> _words;
    private TextMeshProUGUI _content;
    private UnityEngine.UI.Button _leftButton;
    private UnityEngine.UI.Button _rightButton;
    private int _currentPage;
    
    private void Awake()
    {
        _currentPage = -1;
        _content = gameObject.transform.Find("Content").transform.GetComponent<TextMeshProUGUI>();
        _leftButton = gameObject.transform.Find("PreviousButton").gameObject.transform.GetComponent<UnityEngine.UI.Button>();
        _rightButton = gameObject.transform.Find("NextButton").gameObject.transform.GetComponent<UnityEngine.UI.Button>();
        _words = new List<string>();
        gameObject.SetActive(false);
        _leftButton.enabled = false;
        _rightButton.enabled = false;
    }

    public void Close()
    {
        _currentPage = -1;
        _words.Clear();
        gameObject.SetActive(false);
        _leftButton.enabled = false;
        _rightButton.enabled = false;
    }

    public void AddWord(List<string> newWords)
    {
        _words.Clear();
        
        foreach (var newWord in newWords)
        {
            _words.Add(newWord);
            //_words.Add(API(newWord));    
        }

        if (_words.Count <= 0)
        {
            return;
        }
        
        gameObject.SetActive(true);
        
        _currentPage = 0;
        _content.text = _words[_currentPage];

        if(_currentPage < _words.Count - 1)
        {
            _rightButton.enabled = true;
        }
    }

    public void NextPage()
    {
        _currentPage += 1;
        _content.text = _words[_currentPage];
        if (_currentPage == _words.Count - 1)
        {
            _rightButton.enabled = false;
        }

        _leftButton.enabled = true;
    }

    public void PreviousPage()
    {
        _currentPage -= 1;
        _content.text = _words[_currentPage];
        if (_currentPage == 0)
        {
            _leftButton.enabled = false;
        }
        _rightButton.enabled = true;
    }
}
