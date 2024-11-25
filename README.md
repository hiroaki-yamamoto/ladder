# Ladder marker for Doller Cost Average (DCA) strategy

[![TestImg]][Test]

[TestImg]: https://github.com/hiroaki-yamamoto/ladder/actions/workflows/test.yml/badge.svg
[Test]: https://github.com/hiroaki-yamamoto/ladder/actions/workflows/test.yml

## Overview

This code helps your DCA stock trading strategy. It provides ladder pairs that
contain buying price and amount.

## Why API call and auto-trading not supported?

1. Japanese financial platform (e.g. SBI) does not provide API for stock trading.
2. [Alpaca], which is available in the US, but it provides the market data **not**
  in real-time, in cheaper plan.
3. Stock trading is not optimized for HFT (technology, legal stuff, etc...)
  So, manual trading is enough for DCA.

[Alpaca]: https://alpaca.markets/

## How to use
This app runs in terminal. You can build the binary code from source code, and
you will see the following help message when you run the binary code with
`--help` cmdline argument.

```shell
./ladder --help
Calculate the ladder price and qty to buy for stock trading (aka. DCA calculator)

Usage: ladder [OPTIONS] <LOW> <HIGH>

Arguments:
  <LOW>   The minimum price to end the ladder
  <HIGH>  The maximum price to start the ladder

Options:
  -n, --num-ladder <NUM_LADDER>
          The number of ladders to calculate [default: 4]
  -b, --bese-budget-per-trade <BESE_BUDGET_PER_TRADE>
          The base budget per trade [default: 1000]
  -w, --weight <WEIGHT>
          The weight to increase / decrease the budget per trade. The buget when trading is calculated as bpt = previous bpt * weight [default: 1.0]
  -h, --help
          Print help
  -V, --version
          Print version
```

## How to Report Issues

Reporting issue is **not** welcome, but sending Pull Request is welcome.

## License

Check the [LICENSE](LICENSE.md) file.

## Conclusion (Just My Opinion about trading)

This is the philosophy I keep in mind when making investments: At the base of
human thought lies the will, a primal instinct of animals. As long as the will
exists, animals continue to affirm their own will. In the survival-of-the-fittest
capitalist society, this affirmation of the will becomes particularly pronounced.

The affirmation of the will means that its manifestations—desires, or
cravings—are also affirmed. And because the will is boundlessly free, the
degree of this affirmation knows no limits. When the affirmation of desire
reaches its highest level, a person’s desires may drive them to acts that force
others—or even themselves—to deny their own desires and, at their root, their
will. In other words, a will affirmed to the highest degree can lead to the
killing of oneself or others. Ultimately, it is difficult for humans to live
without, to some extent, killing others.

If you can make good use of this idea, you should be able to make money too.
