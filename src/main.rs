use chrono::{DateTime, Local, Months};
use clap::Parser;
use currency::{Compounded, Currency, Interest};
use tabled::{
    settings::{
        style::{HorizontalLine, On},
        Style,
    },
    Table, Tabled,
};

mod currency;

struct Entries {
    start: Entry,
    entry: Option<Entry>,
}

#[derive(Clone, Copy, Debug, Tabled)]
struct Entry {
    #[tabled(display_with = "display_date")]
    month: DateTime<Local>,
    amount: Currency,
    interest_accumulated: Currency,
    interest_rate: f32,
    payment: Currency,
}

fn display_date(date: &DateTime<Local>) -> String {
    format!("{}", date.format("%B %d, %Y"))
}

impl Iterator for Entries {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.entry.or(Some(self.start))?;

        let month = current.month + Months::new(1);
        let amount = (current.amount - current.payment) + current.interest_accumulated;
        let interest_accumulated = current
            .amount
            .interest(Interest::new(current.interest_rate, Compounded::Monthly));
        let interest_rate = current.interest_rate;

        let payment = if amount - current.payment < Currency::new(0.0) {
            amount
        } else {
            current.payment
        };

        self.entry = if self.entry.is_none() {
            Some(self.start)
        } else if amount > Currency::new(0.0) {
            Some(Entry {
                month,
                amount,
                interest_accumulated,
                interest_rate,
                payment,
            })
        } else {
            None
        };

        self.entry
    }
}

fn entries(current: Currency, interest: f32, payment: Currency) -> Entries {
    Entries {
        start: Entry {
            month: Local::now(),
            amount: current,
            interest_accumulated: current.interest(Interest::new(interest, Compounded::Monthly)),
            interest_rate: interest,
            payment,
        },
        entry: None,
    }
}

const STYLE: Style<On, On, On, On, On, On, 0, 0> = Style::rounded()
    .line_horizontal(HorizontalLine::inherit(Style::modern()))
    .remove_horizontals();

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    current: Currency,

    #[arg(long)]
    interest: f32,

    #[arg(long)]
    payment: Currency,
}

fn main() {
    let args = Cli::parse();
    let entries = entries(args.current, args.interest, args.payment)
        .take_while(|x| x.amount > Currency::new(1000.0));
    let mut table1 = Table::new(entries);
    table1.with(STYLE);
    println!("{}", table1)
}
