import sys
import math

class MarketData():
  def __init__(self, price, quantity):
    self.price = float(price)
    self.quantity = float(quantity)

class TradeRequest():
  def __init__(self, tradeRequestTokens):
    self.qty = int(tradeRequestTokens[0])
    self.riskPerQty = float(tradeRequestTokens[1])

def ParseMarketData(mdLine):
  tokens = mdLine.split()
  return [MarketData(price, size) for price, size in zip(tokens[1::2], tokens[::2])]

class Hedger():
  def __init__(self, bids, offers):
    self.bids = bids
    self.offers = offers

def PassTrade(self, tradeRequest):
  # todo - candidate
  pass

linesParsed=0

hedger = None
for line in sys.stdin:
  if linesParsed == 0:
    offers = ParseMarketData(line)
    linesParsed += 1
  elif linesParsed == 1:
    bids = ParseMarketData(line)
    hedger = Hedger(bids, offers)
    linesParsed += 1
  else:
    if hedger is None:
      raise Exception('no good')
    tradeRequest = TradeRequest(line.split())
    res = hedger.PassTrade(tradeRequest)
    if res is not None:
      print(res)
      linesParsed += 1