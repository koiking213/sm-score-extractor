# sm-score-extractor
Extracts highest score from StepMania's Stats.xml.
Player ID information file is required as ids.json.

Example of ids.json:
```json
{
  "players": [
    {
      "id": "aaaabbbbccccdddd",
      "name": "player1"
    },
    {
      "id": "eeeeffffgggghhhh",
      "name": "player2"
    }
  ]
}
```

Output format:
```json
[
  {
    "name": "xxx",
    "charts": [
      {
        "difficulty": "Hard",
        "scores": [
          {
            "score": 999990,
            "player": "player1"
          }
        ]
      }
    ]
  }
]
```
