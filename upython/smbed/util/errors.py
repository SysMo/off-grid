import io
import sys

global_context: list[str] = []

class CatchErrors:
  def __init__(self, logger, context: str):
    self.logger = logger
    self.context = context

  def __enter__(self):
    global_context.append(self.context)
    return self
    
  def __exit__(self, exc_type, exc_value, traceback):
    if exc_value is not None:
      sb = io.StringIO()
      sys.print_exception(exc_value, sb)
      self.logger.error(sb.getvalue())
      self.print_context()
    global_context.pop()
    return True
  
  def print_context(self):
    self.logger.error("Error context:")
    for (i, ctx) in enumerate(global_context):
      self.logger.error(f"{i}: {ctx}")