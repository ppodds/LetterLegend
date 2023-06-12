using System;
using System.Collections.Generic;
using System.Net.Http;
using TMPro;
using UnityEngine;
using System.Threading.Tasks;

public class Dict : MonoBehaviour
{
    private List<string> _enWord;
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
        _enWord = new List<string>();
    }

    public void Close()
    {
        gameObject.SetActive(false);
    }

    public async void AddWord(List<string> newWords)
    {
        foreach (var newWord in newWords)
        {
            if (_enWord.Contains(newWord))
            {
                continue;
            }
            var trans = await GetContent(newWord);
            _words.Add(newWord + ": " + trans);
            _enWord.Add(newWord);
            gameObject.SetActive(true);
        }

        if (_words.Count <= 0)
        {
            return;
        }

        _currentPage = 0;
        _content.text = _words[_words.Count - 1 - _currentPage];
        
        if(_currentPage < _words.Count - 1)
        {
            _rightButton.enabled = true;
        }
        else
        {
            _rightButton.enabled = false;
        }

        _leftButton.enabled = false;
    }

    public void NextPage()
    {
        _currentPage += 1;
        _content.text = _words[_words.Count - 1 - _currentPage];
        if (_currentPage == _words.Count - 1)
        {
            _rightButton.enabled = false;
        }

        _leftButton.enabled = true;
    }

    public void PreviousPage()
    {
        _currentPage -= 1;
        _content.text = _words[_words.Count - 1 - _currentPage];
        if (_currentPage == 0)
        {
            _leftButton.enabled = false;
        }
        
        _rightButton.enabled = true;
    }

    private async Task<string> GetContent(string str)
    {
        var client = new HttpClient();
        var request = new HttpRequestMessage
        {
            Method = HttpMethod.Post,
            RequestUri = new Uri("https://google-translate1.p.rapidapi.com/language/translate/v2"),
            Headers =
            {
                // TODO: change api key to env
                { "X-RapidAPI-Key", "" },
                { "X-RapidAPI-Host", "google-translate1.p.rapidapi.com" },
            },
            Content = new FormUrlEncodedContent(new Dictionary<string, string>
            {
                { "q", str },
                { "target", "zh-TW" },
                { "source", "en" },
            }),
        };
        
        using (var response = await client.SendAsync(request))
        {
            response.EnsureSuccessStatusCode();
            var body = await response.Content.ReadAsStringAsync();
            var index = body.IndexOf("\"translatedText\":\"");
            var returnStr = "";
            for (var i = index + 18; i < body.Length; i++)
            {
                if (body[i] != '\"')
                {
                    returnStr += body[i];
                }
                else
                {
                    break;
                }
            }
            return returnStr;
        }
    }
}
