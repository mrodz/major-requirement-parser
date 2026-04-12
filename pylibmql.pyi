"""pylibmql.pyi

Transform MQL input to structured JSON output. 

```
import pylibmql
    
output = pylibmql.parse('SELECT 1 FROM CLASS(MATH 2250): "Must take MATH 2250";')

print(output.version())
'''
0.1.3
'''

print(output.json())
'''
{"version":"0.1.3","requirements":[{"query":{"quantity":{"Single":1},"selector":[{"Class":{"department_id":"MATH","course_number":2250}}]},"description":"Must take MATH 2250","priority":1}]}
'''

print(output.json_pretty()) 
'''
{
  "version": "0.1.3",
  "requirements": [
    {
      "query": {
        "quantity": {
          "Single": 1
        },
        "selector": [
          {
            "Class": {
              "department_id": "MATH",
              "course_number": 2250
            }
          }
        ]
      },
      "description": "Must take MATH 2250",
      "priority": 1
    }
  ]
}
'''
```
"""

from typing import Dict

class MQL:
    """
    Structured output from an MQL parse
    """
    def version(self) -> str: 
        """Return the version of the parser used to create the structured output."""
        ...
    def json(self) -> str: 
        """Return the structured output as JSON."""
        ...
    def json_pretty(self) -> str: 
        """Return the structured output as pretty-printed JSON."""
        ...

def parse(mql: str) -> MQL:
    """
    Parse MQL input to a structured output.
    This can be formatted as JSON, pretty-printed JSON, or the internal Rust struct with `str`
    """
    ...

def parse_with_externals(mql: str, externals: Dict[str, str]) -> MQL:
    """
    Parse MQL input with external variable substitutions.
    
    Args:
        mql: The MQL query string to parse
        externals: Dictionary mapping variable names (including $ sigil) to their values as MQL expression strings
        
    Returns:
        MQL object containing the parsed result
        
    Example:
        output = pylibmql.parse_with_externals(
            'extern $course; SELECT 1 FROM $course: "Take the course";',
            {"$course": "MATH 2250"}
        )
    """
    ...

def parse_extern_value(expr: str) -> str:
    """
    Parse an MQL expression string into a value suitable for use as an extern variable.
    
    Accepts the same syntax as variable assignment values:
    - "MATH 2250" → Class selector
    - "[MATH 2250, MATH 2260]" → Selector list
    - '"some string"' → String value
    - "3" → Quantity value
    
    Args:
        expr: MQL expression string to parse
        
    Returns:
        Internal representation of the parsed value (for debugging/inspection)
    """
    ...
