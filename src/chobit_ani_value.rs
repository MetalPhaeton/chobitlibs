// Copyright (C) 2023 Hironori Ishibashi
//
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See http://www.wtfpl.net/ for more details.

#![allow(dead_code)]

//! Utility for UV animation.
//!
//! This helps to calculate UV coordinates for animation.  
//! All coordinates are calculated and saved at [ChobitAniValue::new()],
//! so it's very fast to get UV coordinates.
//!
//! For example, from the following texture...
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAACACAQAAABOvH+BAAAhi2lDQ1BpY2MAAGiBpZN3VBRBs+h7
//! ZnOGhSWHJbPknHPOWXKQsOScEZCsIIpKliCiJBEUEBBFAVFQURFBBRVBRQUMGAFFwYvnu+e8d867
//! 969Xc7qqpqe6qqb71wAIN0UGRiXALABERSfGmzsZerh7eNIxi4AA8IAIqEDDPzAhVio6LBX8r7Lx
//! BED/7Izsv1z/e9z/KMggZkLgru3dHVcDY+MTAYBidwc1JTF214fzd+fZI1ycjHf9agDoxP+O/yd0
//! 83j/NFkTVQVZZmQkU9ZVSTYhPiQgMT5QLiww8P+qIQTMQTzwB2lAFpgAVaCwa5kgcvdh7nquQGlX
//! J+xGhIAAkLhrA4EcCNvVgf9Pt/+fkshMTfxnjWNi0+LDQkIT6UoKiqoydNPd9unOiTHRTLpkCjMg
//! ISyRqUUPTUyM1ZKXjw6LZgYxQ+KZzIQAZmRMilxgTJS8Np0Z5R8WqUX/9+MJ/xYa/E9xDDm6S2hY
//! At3S2JgeGx8THLZbZvc1MiyQGZ3ADKInRQcx4+n+dON4pn9iWDKTbhwTFRUTnUA3TEyMDwtISgyL
//! iZZ1DvWPZxpGhkUw6cpyCvQ90bEx8Ym7i23+k4Uu+a/RhN1OA/87S+B/ksjFxIfI/3epBPmANNkE
//! f/ndBPKRzBD/yMCYICZD7t9e/OPtP7vzfzgKTIpP/s8cyz8FARggAQqgAQbgdrkk7JJJBpTdj6yA
//! DbADGuAAXIAb8ABewAcEgCCg7565CBAFYkAcSAIGkALSu2csB+R3z14JKAOVXQrUgQbQBFpAG+gC
//! PaAPDIARMN7lw3SXFgtgCayADbAFdsAeOAIn4AxcwB7gBtyBB/AE3sAH+AK/XaYCdikJAsG77ITu
//! MhOxy1QUiAYxIG6Xo4RdmpJBCkjdJS8dZIBMsB9kgxyQC/JAATgADoJCUASKwWFwBJSAY+A4KAVl
//! oAJUgipQDWpALagD9aABnAKN4DRoAs2gBbSCNtAOzoEO0AkugC7QDXp2704f6AeXwGVwBQyCq2AI
//! DIMRcB2MgptgDIyDW+AOmAB3wT0wCR6AKfAQzIBH4DGYBXPgKXgG5sELsABeglfgNXgD3oJ3YAWs
//! gvfgI/gEPoMv4Cv4Dn6ADbAJfoIt8Btsgx3wF4IgGEJCKAgDYSE8RIBIEBmiQKwQFWKHaBAnxAXx
//! QLwQPyQI0SFhSAQSg8QhSYgBSUOykBykAClCypAKpAZpQJqQNqQD6UEGkCFkDJlCZpAFZAVZQ7aQ
//! PeQAOUEu0B7IDfKAPCFvyBfaC/lDgRATCoFCoXAoEoqGYqA4KAFKgpKhVGgflAHth7KgHCgPKoAO
//! QkXQIegwVAIdg0qhcqgSqoJOQLVQPdQANUJnoGaoFToLnYM6oPNQF9QD9UL90AB0BboKDUEj0Ch0
//! ExqHbkMT0D1oEpqCpqFH0BNoDnoGzUML0CtoCXoLLUOr0AfoE/QZ+gp9h9ahn9AW9AfagQEMw0gY
//! A+NgAkyCKTAVZoc5YC6YB+aHBWEhWAQWgyVhKVgGloMVYWVYFdaAtWAdWA82hI1hU9gCtoJtYHvY
//! EXaGXWF32BP2gf1gfzgIDobD4Ag4Co6F4+EkOAVOgzPg/XAOnAcfgAvhYvgIfAwuhSvgKrgGroMb
//! 4Ea4CW6Bz8Ln4PNwF3wR7oMH4EH4GjwCj8Jj8G14Ar4PP4Cn4cfwLPwMfgEvwq/hN/Ay/B7+CH+G
//! v8Hr8Ca8BW/DfxEwAoXAIvAIEoIFwYagIbgQvAgBBB0hghBHMBAyCDmEIkIFoY7QQugi9BFGCFOE
//! BcIaYYdwRLggXBEeCG+EHyIAwUSEIiIQ0Yg4RCIiBbEPkYnIRuQiChCFiGJECeI4ohxRjahFnEQ0
//! IpoQrYh2RCeiC3ER0Y+4jLiKGEaMIsYQdxD3EA8Q04jHiDnEc8Qi4jXiLWIF8QGxhviGWEf8RPxG
//! 7CBhJAqJRRKQFCQVSUNyIfmQgkhhpBiSgZRByiOVkWpITaQu0gBpjDRHWiFtkY5IF6Qb0gvpiwxA
//! MpGhyEhkDDIBmYxMQ2Yis5H5yIPIYmQJshRZgTyBrEOeQp5BtiLbkeeR3cg+5ADyKnIYeQM5jpxA
//! TiIfIh8j55DzyJfIJeQy8j1yDfkNuY78hdxGARQShUURUBQUG4oDxYMSQAmjxFAMlCxKEaWC0kDp
//! oAxQxihzlDXKHuWEckV5onxRAahgVBgqChWHSkKloTJROah8VCHqMOoYqhxVjapDnUI1odpQHagL
//! qIuoS6hB1DDqBuoW6i7qAWoGNYt6jlpELaGWUR9Qn1E/UD9Rf9AAjURj0UQ0C5odzYXmQwuhxdAM
//! tCxaEa2K1kTroY3QZmgrtB3aGe2G9kL7oQPRoehIdCw6EZ2GzkTnoAvQh9Al6FJ0FboW3YBuQreh
//! O9Dd6D70ZfQQehQ9jr6LfoB+hJ5Dz6Nfod+i36PX0N/Qm+jfGIBBYnAYEoaK4cDwYgQxohhJjCxG
//! EaOG0cLoY4wxFhhbjCPGFeOJ8cMEYkIxUZg4TDImHZOFyccUYY5gSjFVmFrMKUwz5izmPOYi5hLm
//! KuY6ZhwzgXmAeYR5inmBeY1ZxnzEfMWsY7Ywf7FILA5LwlKxnFg+rBBWHCuNlceqYDWxelhjrAXW
//! FuuEdcN6Y/2xwdgIbCw2CbsPm4XNxxZhS7Bl2GpsPfY0thXbge3G9mMHsSPYMewE9gH2EfYpdhH7
//! BruKXcN+x/7EbuMQOCyOhKPiOHH8OGGcBE4Gp4hTx+ngDHFmOBucI84V54XzxwXjInBxuGRcOi4H
//! dwBXjDuOq8TV4hpxLbhzuG5cP+4q7jpuHHcP9xA3i5vHvcYt4z7hvuF+4rbxMB6LJ+HZ8Nx4Abwo
//! noGXx6vgtfD6eFO8Fd4B74r3wvvjg/GR+Hh8Cj4Tn4cvxJfgy/En8A34Znw7vgvfj7+Kv46/hb+P
//! n8HP4Rfwb/Dv8Z/x6/jfBIiAIRAJVAIXQYAgSpAiyBNUCdoEQ4I5wZbgTPAg+BGCCOGEWEIyIYOQ
//! SygklBDKCTWEU4QWQgehhzBAGCLcJEwQpghPCPOE14RVwmfCOuE3ESJiiCQiG5GbSCeKE2WISkQN
//! oh7RlGhNdCS6EX2IQcRwYiwxmZhJzCMWEY8SK4l1xNPENuIFYh9xkDhKvE2cJD4mPie+Iq4Q14jr
//! xN8kiIQlkUk0Ei9JiCRJkiOpkrRJRiQLkj1pD8mbFEAKI8WQkkkZpDxSEekYqZJUT2oitZO6SZdI
//! Q6Qx0l3SNOkpaZH0jvSR9J20RYbIGDKZTCPzkoXJDLI8WY2sSzYhW5Edye5kP3IwOZKcQN5HziEX
//! kkvIFeQ68hlyO7mbPEAeIo+R75FnyM/Ir8gr5M/kDfI2BUkhUKgUbgqdIkGRo6hSdCjGFCuKI8Wd
//! 4kcJpkRREinplFxKEeUYpZrSQGmhdFJ6KYOUUcoE5SFljrJIeUf5RFmn/GFBsOBZqCzcLHQWCRZ5
//! FjUWXRYTFhsWZxZPlgCWMJZYllSWLJYDLCUsFSz1LE0s51guslxhGWW5wzLFMseyyLLMssaywbLN
//! imIlsrKz8rKKsEqxKrFqshqyWrA6sLqx+rGGsEazJrNmshawHmEtZ61jbWI9x3qRdZB1lHWCdZr1
//! Kesr1lXWL6w/qYCKpVKonFRBqgRVjqpG1aOaUe2oe6g+VCY1ippEzaTmUw9Ty6l11CZqB7WXepV6
//! k3qP+oj6nLpE/UD9Tv3NhmAjsLGx8bKJsEmzKbNps5mw2bC5sHmzBbFFsiWyZbDlsx1mK2erZ2tm
//! 62TrY7vGNsZ2n+0J2wLbO7Y1tg22v+wYdgo7JzudXZJdgV2D3ZDdkt2R3ZM9gD2cPYE9nT2P/TB7
//! OXsdezN7J3s/+xD7OPsD9ln2l+wr7F/Zf9EgGp5GpfHSRGgyNBWaLs2UZkdzo/nRQmmxtDRaDu0Q
//! rYxWS2uiddD6aEO0cdoD2hztJW2V9o22xYHgIHKwc/BziHPIc6hzGHBYcjhxeHIEckRyJHHs5zjA
//! cZSjmqORo53jIscgx02O+xxPOBY5Vji+cmxxIjiJnOyc/JzinPKcGpyGnFaczpzenEzOaM4UzmzO
//! Is5SzjrOZs7znJc4RzjvcE5zPud8w/mJc5MLcOG4qFy8XKJcclzqXAZcllxOXN5cTK5orhSuHK5D
//! XGVcdVwtXBe4BrhGue5yPeJa4Frm+sK1xY3gJnJzcAtyM7iVuLW5TbntuN25/bkjuBO593Mf5D7O
//! XcPdxN3JfYl7hHuCe4b7Bfc77i/cWzwIHhIPBw+dR4pHmUeXx4zHgceTJ5AniieFJ4fnEE85z0me
//! Vp5unkGeMZ5Jnjme1zwfedZ5/vLieNl4+XkleBV4tXhNeO143XkDeCN5k3izeIt4y3jreVt5u3kH
//! ecd4H/A+5V3i/cS7yQfxEfhofIJ8DD5lPl0+cz5HPi8+Jl8s3z6+fL4Svmq+03wdfP181/nu8j3m
//! W+Rb5fvBt8OP5afy8/FL8Cvya/Ob8jvwe/IH8cfwp/Hn8ZfwV/Of5u/kv8R/nf8e/xP+V/wf+NcF
//! gABegCYgKCAloCKgJ2Ap4CzgKxAqkCCQKVAoUCZQL9Am0CNwTeCWwLTAvMCywDeBP4IYQaogn6CE
//! oJKgjqC5oJOgt2CIYLxghuBBwVLBesFWwR7Ba4K3BacFXwiuCH4X3KFj6Wx0ATqDrkLXp1vRXeh+
//! 9HB6Ej2bXkyvpDfSO+iX6KP0+/Q5+hJ9jf5LCClEEeIREhNSENIWMhNyFPIWChGKF9ovVCRULtQg
//! 1C7UJzQidE9oVui10Cehn8JIYYowj7CYsIKwjrC5sJOwr3CYcKJwlnCxcKVwo3Cn8IDwDeEHws+E
//! 3wl/Ff4jghVhExEUkRJRFTEUsRFxFwkUiRHZJ3JA5LhIvUibSK/IsMhdkScir0XWRH6JokRZRPlE
//! JUWVRfVFrUVdRQNEo0XTRAtEj4vWibaJ9ooOi94VnRVdEl0T3RJDi1HFBMQYYqpihmK2Yh5iTLFY
//! sQyxQrFysVNiHWIDYjfFpsTmxZbFvov9FSeIc4qLiMuLa4ubizuL+4lHiKeI54sfE68TbxPvFR8R
//! vyc+J/5G/Iv4tgROgiYhJCEroSlhJuEk4SsRLpEskSdxVKJWolWiV2JE4p7EU4m3El8ltiXxkhyS
//! IpLyktqSFpIuknsloyTTJA9IlkqelDwneUnypuSU5AvJVckNBsygMHgZkgwVhiHDluHJCGEkMLIZ
//! hxknGC2MHsYw4x5jjvGW8ZWxI0WQ4pISlVKU0pOyknKTCpKKk8qUOiRVJdUk1S11TWpCalbqjdRX
//! qR1pvDSntKi0orSetLW0uzRTOl46S/qw9AnpZuke6WHpe9JPpd9Jf5eBZMgyvDKSMqoyRjL2Mt4y
//! YTLJMnkyx2ROypyTGZAZk5mWWZT5JLMli5FllxWSlZfVkbWUdZUNko2T3S9bLHtCtkX2oux12UnZ
//! 57KrshtySDlWOQE5GTlNOXM5Fzl/uRi5DLlDclVyzXI9ciNy9+Wey63Ibcgj5FnlBeRl5LXkzeX3
//! yAfIx8rvly+WPyHfKt8rPyo/Jf9C/oP8LwW0AruCsIK8gq6CtYKHQohCokKuwjGFkwodCpcVbik8
//! VlhS+KrwV5GkyKMoqaimaKLopLhXMVoxQ/GQYrVii2Kv4qjilOKC4kfFLSWsEoeSqJKSkoGSnZK3
//! UrhSmtJBpQqlM0rdSsNK95Xmld4r/VRGK7MriygrKusr2yp7K4crpyofUK5QPqPcrTyiPKk8r/xB
//! +ZcKRoVDRVRFWcVQxV7FVyVSJV2lSKVapUWlT+WGyrTKS5XPKtuqRFUeVYaquqqZqotqoGq8arbq
//! UdV61Q7VQdU7qnOqy6rrakg1NjUhNQU1fTU7NW+1CLV9akVqVWotan1qN9Vm1F6pfVH7q05W51eX
//! UddWt1R3Vw9RT1YvUC9XP6Peo35dfUp9UX1NfVuDqMGrIaWhqWGh4aYRrJGkka9RpnFao1vjusaU
//! xqLGmsa2JlGTV1NaU0vTUtNdM0QzRfOAZoVmk2av5g3Nac1Xml+1gBZFS1BLTktXy1bLWytCK13r
//! kFaN1lmtAa3bWrNay1ob2mhtmraYtoq2ibazdqB2vHau9nHtRu0u7RHtKe1F7c/aOzpkHQEdWR1d
//! HVsdb51InQydYp1anXM6V3QmdJ7pvNf5pYvT5dZl6GrqWui664bqpuoW6lbrtupe0r2lO6u7rLup
//! h9Hj1JPQU9cz13PTC9FL0TuoV6XXqndJ75berN6y3qY+Rp9TX0JfXd9c300/RD9Vv1C/Wr9Nf0D/
//! tv5T/VX9XwY4A24DKQMtAysDT4MIg3SDYoM6gw6Dqwb3DV4YfDLYMSQbChjKG+ob2hvuNYw1zDE8
//! bnjasMfwhuGM4RvDH0YoI5qRuJGakbmRm1GoUapRkVGNUbvRoNFdo3mjT0bbxmRjQWN5YwNjR2N/
//! 43jjPONy4ybjPuNx41njZeOfJjgTHhNpE20TGxMfk2iTLJNjJo0mPSajJjMmb0zWTdGmnKaSppqm
//! VqZeppGmmaYlpg2mXabXTadNl0x/mKHMOM0kzTTNrMy8zCLNMs2Omp0y6zYbNZsxe2O2YY4x5zJn
//! mGub25j7mEebZ5sfNz9t3ms+Zv7EfMX8lwXegs9C1kLfwsHC3yLBIt+i0qLV4rLFhMW8xSeLv5Ys
//! lsKWypamlq6WoZb7LIst6y3PW45YTlsuWa5boa24rKSstK1srfys4qxyrcqtWqwuWd2xem71yeqv
//! NYu1sLWKtZm1u3WYdbr1EesG627rG9aPrZetf9ngbfhs5GwMbJxsgmySbQptamw6bIZspmxe2/yw
//! Rdty20rb6tra2/rbJtgW2FbZnrW9ajtp+9L2mx3SjtOOYadtZ2vnZxdvl29XaXfWbtDuvt2i3Td7
//! pD2HPcNe297Ofq99vH2BfZV9u/1V+0n7V/bfHdAOXA7SDroODg4BDkkOBx1qHDodhh2mHd44bDri
//! HPkc5R0NHV0cQxzTHA87Njh2O950nHVcdfzjRHESdlJxMnfydIpyynYqdWp2GnC667Tg9NUZ6czh
//! zHDWcbZ3DnBOci50rnU+73zd+ZHzsvOWC8mF7qLsYubi4RLpkuVS6tLsMuBy12XB5dse1B6uPdJ7
//! 9PY47mHuSd1TvOfknu49Y3vm9nzYs+NKdRVz1XC1dvVzjXctcD3h2uE67Drj+s51y43kRndTcTN3
//! 83KLdst1q3A763bVbcptyW3THe8u4K7oburu7h7pnu1e5t7qfsV90v21+7oHzoPfQ8HDxMPdI8Ij
//! y6PMo9Xjisekx2uPDU+cJ7+noqepp4dnpGe2Z7lnm+dVzynPN54/vQhegl7KXuZeXl4xXnleVV7n
//! vIa9ZryWvX57U7xFvNW9rb33eid6F3rXeXd53/Se8/7oA/nQfBg+uj6OPkyffT4lPqd9+n3u+iz6
//! fPfF+vL5Kvia+Hr4Rvnm+lb6tvsO+874Lvv+8WPxE/PT9LPzC/BL8Sv2O+XX63fH74Xft73ovbx7
//! 5fea7HXfG7U3d2/l3nN7h/c+2ru6d9uf6i/hr+3v4M/03+df4n/Gf8D/vv8r/40AQgA9QDXAMsA3
//! ICGgMKA+oCfgVsDzgC+BqECeQPlAk0CPwOjAvMDqwM7A0cDZwI9BUBBHkHSQQdCeoPCgrKCKoPag
//! 4aBHQatBO0w2JoOpx3RmhjIzmWXMNuY15jRzhbkdTA2WCNYNdgoOCc4MLg1uDb4WPB28ErwdQg2R
//! DNENcQ4JDdkfUhZyNmQo5FHIasjfUPZQqVD90D2h4aHZoZWh50Kvhz4J/RgGh3GGyYYZh7mHRYfl
//! h50IuxA2FvY87Es4OpwvXCncItwnPCG8KLwhvC98Ivxl+EYEMUI4QiPCLiIoYl/EsYiWiKsR0xEr
//! ETuR7JFSkQaRrpGRkbmR1ZHnI29GPov8EoWO4otSjrKM8otKiiqOaowaiJqMehO1Fc0aLRGtG+0c
//! HR6dHV0Z3Rl9I/pp9JcYdAxfjHKMZczemOSYwzFnYi7HTMW8i9mOZYuVijWIdYuNis2PrYntjr0V
//! uxD7I44QJxynEWcfFxyXEVcW1x43EjcbtxaPjOeNV4q3iPeLT44/Et8UfyX+YfxK/N8EjgSZBOME
//! z4S4hMKEhoT+hPsJbxK2ElkTJRP1E10ToxLzE2sTexLvJL5M3EwiJ4kn6Sa5JEUk5SadSOpKupW0
//! kLSRTEoWTdZOdkoOT85Jrk6+kHwreSF5PYWYIpqineKUEp6Sk1Kd0pVyK2UhZSOVlCqWqpPqkhqR
//! mptak9qdeif1ZerPNEqaRJpemmtadFpBWn1ab9q9tKW03/vY9kntM9rnuS9uX9G+xn0D+6b2raSD
//! dM50+XTzdL/05PSS9Jb0ofQn6WsZqAz+DNUM2wxmRmZGRUZnxljGfMaPTGKmaKZOpktmZGZ+Zl1m
//! b+b9zLeZ2/tp+2X3m+732Z+0/8j+5v1D+5/sX8tCZwlkqWfZZ4VkZWVVZXVl3c56mfUrmzWbkW2Y
//! 7Zkdn30o+0z2YPaj7I85yBz+HNUcu5zgnKycqpyunNs5r3J+5VJzpXKNc71yE3MP5zbnDuU+yf2c
//! h8mj52nmOeaF5+Xl1eb15t3Pe5u3k8+Zr5Bvke+fvy+/LL8j/2b+i/yNAkqBZIFhgWdBfEFxQVPB
//! tYInBZ8PYA8IHdA64Hwg8kDBgZMHLh2YOrB6ED7Ie1DloO3BkIPZB08c7Dl49+Cbg9uFnIUKhZaF
//! AYXphRWF5wtvFb4s3CpiK5IpMivyK0otOl50ruhm0ULR5iGWQ1KHjA/5HEo+dPTQ2UOjh+YPrRdT
//! iiWLjYq9ipOKS4rbikeLnxevHyYfljxsdNjrcNLho4fbDo8enj+8foRyhHHE6Ij3keQjx46cPXLj
//! yIsjmyWsJdIlJiW+JaklpSUdJeMlL0u2jrIdlT1qftT/aPrRiqMXjt45unR0+xjnMcVj1seYx7KO
//! 1RzrPTZ5bOU4fJzvuNpxh+MRxwuONxy/fHzm+FoptlS4VKfUrTSutLi0pXSk9FnpjzJKGaPMpMy3
//! LLWsrKyz7HbZ67I/5ZzliuU25cHlOeW15f3lU+UfKlAV9Artij0VsRWHKporRiqeVaxXUiqlKk0r
//! /Sr3VVZUdlVOVL6tAlW8VapVDlURVQeqGqsGq55Ufa0mVktUG1X7VKdUl1Z3Vt+uXqreOcF9QuWE
//! /YnwEwUnTp0YPPHkxNcaYo1EjVGNb01aTXnNhZqJmre1oJavVr3WqTaqtrC2qXa49lnteh1rnUyd
//! eV1g3f66E3V9dVN1H+ox9cL1uvUe9Yn1x+rP1Y/Xv67fPsl9UuWkw8mIkwdPnj45dPLZyfUG1gaZ
//! BouGwIashtqG/obphrVTuFNipwxPeZ9KPVV+quvU3VPLjYhGwUatRtfG+MaSxvbG8cZXjdunuU+r
//! nnY8HXW66HTz6eunX5z+dYZ2RvGM7ZmwMwVnGs9cO/P0zHoTa5Nsk2UTsymn6WTTlaYnTd+ayc1S
//! zebNgc1ZzbXNA82Pmr+0EFsYLaYt/i37W2pa+ltmWj63ElolW01a97Zmtp5o7W+dbv3cRmiTbDNp
//! 82/LbKtp62+baft8lniWcdb0bMDZ/Wdrzw6cfXT2azu5XardvD2oPbu9vv1K+2z793Ms52TPWZ0L
//! OZd37tS5a+eendvsYO9Q6LDtCO842NHUcb1joeN3J1enaqdTZ0zn4c6zneOdS+fBef7zWufdziee
//! Lz1/4fy986sX0BdELhhc8L2QfqH6Qt+FmQtfukhdUl0WXcyu3K6Grmtdz7o2u2ndSt0O3VHdh7rb
//! use6l3pAj0CPdo9HT3JPeU93z4OejxfxFyUuml4MuJh98eTFwYtPL270svcq9dr3RvUW97b1jve+
//! 6YP7BPt0+7z60voq+3r7pvu+9JP7Zfot+0P6C/rP9F/vX+zfvsR7SfOS26WkS2WXui89uPRpgDDA
//! GDAfYA7kDTQODA+8GPh9mfuyxmXXy4mXSy93XZ68/OkK4QrjivmV4Cv5V05fGbmyeGV7kHdQa9B9
//! MHmwYvDi4MPBL1fJV2WvWl8Nv1p4teXq2NWla/A1+jX9az7XMq7VXrt8be7axhBtSHnIaShu6OjQ
//! +aH7Qx+G8cOMYfPh4OGC4abh0eFXI2BEcERvxHskfaRm5PLI3Mjmddp1levO1+OvH7/edf3B9bVR
//! 0qjMqPVo+GjRaNvordF3N1A3RG8Y3wi4kXPj1I2RG4s3dm7y39S56XUz/WbNzcs3525ujnGOqY3t
//! GUsaKx+7ODY99m2cdVxh3GE8Zvzo+Pnx++OfbhFvydyyvhVx69Ct9lt3bq3ext6WvG1+O+T2gdst
//! t8duv72DuiN6x+RO0J28O2fu3LjzegKeEJ4wnPCfyJlonBiZeHkX3KXfNbjrdzfrbsPd4buLd//e
//! E7ynf8/33v57J+8N3Vu4t3Nf4L7efd/7+++fvD90f+H+zqTgpP6k32TWZMPk8OTi5N8H9AcGD/Y+
//! yH5w6sH1B6+moCmhKaOpgKncqdNTN6aWHiIeij40ech8WPCw+eH4w3fTmGmJafPp0OnC6bPTd6bf
//! z+BnpGdsZiJnjsx0zkzOrD2iPFJ45PAo7lHpo55HM49+PGZ/rPrY9XHK46rHA4+fPt56wvNE+4n3
//! k8wnJ58MP1mcBbNCs0azgbP5s82z47PLc9g5xpzVXMTc4bnOucm5z09Znio+dXqa8LT8ad/TJ09/
//! PuN+pvXM61nGs5PPhp+9fA49F35u8pz5/MDztud3nr+fJ8zLztvNx8wfn++ZfzS//oLjhcYLjxfp
//! L+peDL1YXIAWhBdMFpgLBxfaFiYWPi6SFuUXHRbjF8sX+xZnF3+95Hmp89L3ZdbLxpc3Xr59hX4l
//! +crqVeSrI68uvHr46vtr2mv11x6v972uez30+uUSvCS6ZLYUunRoqWNpcunLG+oblTeub1Lf1Ly5
//! +mbhLfRW5K3p25C3RW/Pvb3/9ss76juVd67v0t7Vvrv2bnEZXhZdNlsOWy5e7lyeWv62wr6ivuKx
//! kr5ycmVkZWkVtSq5arUatXp0tWf10erme+73Ou993+e8b3o//n71A+GD3AfHDwkfKj9c/jD/Yeej
//! 0EeTj8Efiz52fHzw8dsn9k/qnzw/ZX469enGp7dr2DXpNbu1uLXytUtrz9a2P9M/G38O/lz0uePz
//! g8/fvtC+aHzx+rL/S+OXsS8rXwlf5b46fk38Wv118OvCN+ib6DeLbxHfSr51f3v87ed33u/63wO+
//! F3w/+/3e9y8/2H6o/fD8kfmj8cfNHyvrhHX5daf15PUT69fWX24gNyQ2rDdiNko3+jeebmxvCm2a
//! bIZuHt7s2pzZ3PzJ+1PvZ8DPAz/bf07+/PqL9kvzl8+vnF/Nv+78+rjFsqWy5b6VsXVq6+bWym/i
//! b4XfLr9Tf9f9Hvn95g/2j8wfhz9Jf078ufbn1TZqm7Ftux2/Xbl9ZXthB96R2LHeidkp3xnYmf8L
//! /or9tfwb9bf0b//f53///hctsW6LM1VKuAAAAAlwSFlzAAAuIwAALiMBeKU/dgAABTRJREFUeNrt
//! XVmy4yAMtKjc/5hzDU3V28px8AIICUnd8/cmiWlotIAM2wYAQF7QwHdZ5Ff8IgR/6id/4E/ZBj8G
//! /75mM9d+KI8EAvEnGfqpJBCKv6AA1LvAygeH4v+So2/og1lRBMH4F69GmN96460/UjghKf5iAiBV
//! +jdTwlk+bcm/BJoWuayAEP92ARDdRCTBfXAw/mUDUqPIzQF/+aysDfDJv/R2AZmk4Kv0dhz+pVtw
//! P/OAtbvA2gcH4/8ab9GuC1KaA9/8i9yMUKO/og92y99nFoAYRIx/vwCYLTvfzgcH49//7WNJBFkM
//! wYGE8l5cBP5l9OGV3ojug0PxLxLPzg3f/MWCQPU5YOuDl+C/CZi94nnm8SoG2FB2/N0PnE0AB87p
//! JLCLQcYk0L0UTMZGcL1NYTsRjjjD4l3/lNEGcPUPaSqC3oY/ofPnWjkwp7EAV8PPuYb/3fhrlYRd
//! pL8KZvhi+FlXAqb8eef6xiRQwsz+DJHAST3wiASK1+F/mB+G9/2jEihO6dOWTQKX/PslUNzSr2bi
//! FFUCt/x7JTBQD7Di7NcMx1bj3ycBUQswaVece/bdLayANf8eCQjVA6zTwaQ+J33zd5QFPJxfFLU+
//! YQ7/l3oDO36YuJb7cNvnvTqAyfwHagK/tyF/F2RogXM6VNsQhP+QC2C7zZglzumIwL+MdsFCHtJE
//! At4xsBm0VrDF2kZ4uWCTVMvCF0TuE0JIfTvYdKw5dzH6RwRAiQpCfun/S2sDToe/hz9JNYTUT+qr
//! q56MTgwks5MK3/uhtR2OS8KSuoHL4dd1AeZxcE0Cim5ggTzgc/jjFoScmN2V6vM98i9SSlSxxzWv
//! u0Q+7pd/8az+M1McvyhMjr/TmsDDjCP6KJOMXRQmx9/hOgDV3oqmuHUAc/mPCUCv0/fHodCT1qjY
//! gAD8i0P1Xw7ISVUcx3AJ8vxJRpmTo+C2p9TfHZ7TRvf8Xw5k39rJRLvVQuVEzR3/4oV+k1cnssnT
//! HfIv6o0beVxnF0wb/gD8SaYDdj9Fc3k/vqWTue3zSfkLCmDqbGs/l5N50wrOPPOn0WZNyCzun0aP
//! Pz/X82fnDwCAc8i4ABiahALgw/hDBKkEUKlMhQTyCICN63EBMeDmUFgAmfkPGwALAOQSAAnlkgAs
//! AIAsoK8dMEGjeLlt+X4hCtseyhbAfiEIC1GpY4CKC8p9Poi+AN4KkAnzL50L2Idg2mEYFqJWCQIR
//! hOcUAB8NAG9koAXaCRDuX08AJ/fW/CVkMAehY4DbI0pV/DC2o5OngUByAVRey8b81xAAXx1EH/bC
//! JghgTRuAhSjzdQDj8JWpP5gFOiwA391Eou0E+OcfEMcCcM9HdwtRM1JfCEBv+Hm+brBY1ekCnhxU
//! QkNOQOkeIOQpq2YBSoeuIWNY1wXQM18++Fo+hr+jM94dwNVavMbBadgLCOkCACcCOAnWEFzFFUCL
//! WSfsCcAFABAAEEcA7WfVwgnAAgBRBXA4HlX9kgZAFE9WAn/X554vs7R/A1hYAD0DOXEV8MzMMN4S
//! niYAG/DjP+7+j1hZkM4x9F5AbWdAbEVedHMY+wTIAgAIAIAAADkB/PlVwzs7Ca49bBawu/zs+sqb
//! 8yqiB7dtQgABshVqSSMBxABAtwD45OYqOrmxFoAFACCA/jBQKnxACAgLAIgK4GNWESYZLAAAAQAQ
//! AOAKLa6bb5ZWWXjplRvvy5b8NizAVYfefnR0Mejr5Be59STecJbMqAAah2NIAiw/Wj+/CAkMuIAn
//! h8PLHCAvU9WH2sBH+A/KZjThhsToOAAAAABJRU5ErkJggg==" />
//!
//! when you want to generate animation of the 1st row and ..
//!
//! <img src="data:image/webp;base64,UklGRkoDAABXRUJQVlA4WAoAAAASAAAAPwAAPwAAQU5JTQYAAAD/////AABBTk1GjAAAAAwAAAEA
//! ABAAAD0AAPQBAANWUDhMcwAAAC8QQA8QFzD/8z//8x+AB1e1bTXOmeohTw0S0puDoIbf+Ye3Fs8m
//! ZREHEf0P7hu4/IVgNhHN9iLk/qmOZlKH4nIWiFq+G0ShOUry4NQmnGq1d0kqA1H9aKoFp90djjzB
//! zybIJf4Zep4BNx5sKs9iE+fHYwIAQU5NRvYAAAAIAAADAAAlAAA5AAD0AQADVlA4TN0AAAAvJUAO
//! EB8w//M//wKBJI791fYIwOgAt7bduNE/CIVGZkeLKMWhM3UwaIXh7EoxG37AxS4hov8TAOC8Yey1
//! TxopoyR1wUZ1rafLkrTL2oJXTZC/C/s2M1zha+EnJl6UCWeNpaMMTurg/cD5y33QWDykyyge2pUV
//! DwPg1GQEgEZJBgXIQKdAWleddqnASVCMk6AvGobOpacFXbBRq1n+Mya/pAzqWAhSO6cVrWvUHCWl
//! Mbra4UXpFMjOKUCjAh4AvMylGQBkvGlq10gp65yBig5leOE22OBl3TOA89bdAABBTk1GjAAAAAwA
//! AAEAABAAAD0AAPQBAAFWUDhMcwAAAC8QQA8QFzD/8z//8x+AB1e1bTXOmeohTw0S0puDoIbf+Ye3
//! Fs8mZREHEf0P7hu4/IVgNhHN9iLk/qmOZlKH4nIWiFq+G0ShOUry4NQmnGq1d0kqA1H9aKoFp90d
//! jjzBzybIJf4Zep4BNx5sKs9iE+fHYwIAQU5NRvgAAAAIAAADAAAlAAA5AAD0AQAAVlA4TN8AAAAv
//! JUAOEBcw//M///MfgAfIte22jb4SKpiIalDCpJXWE1HN5LTGBPw2QcouIaL/EwDgakHbcu08kkMr
//! SNmgrdzX6W5B6m6PGyyzg/jdsC49xR4eTnt52zF+Dz3vnJFL69oprQemNoz5641H5jeHEp5OKanJ
//! VABGCf46gEcOwW8OQDiCTgFCSxYpIcDZgKSfDahJRdO4J9lw2nqkayv47b1fDo1k9pyUYtSklkdK
//! tILDI73kCksORYAoxgF4pADmACzVhkcFgPA3CSmP5JBajI70glS8cmkssNTyAOBqKQcAAA==" />
//!
//! and the 2nd row,
//!
//! <img src="data:image/webp;base64,UklGRqABAABXRUJQVlA4WAoAAAASAAAAPwAAPwAAQU5JTQYAAAD/////AABBTk1GvAAAAAgAAAEA
//! AB4AAD0AAPQBAANWUDhMpAAAAC8eQA8QFzD/8z//8x+AB7eRJClS393/ePA01qz0MqMJa80zyUfT
//! bu4eeRDR/wkAsBojWlwijkyCKyntclI7lP8OV1LaOTJph+KC6G4SyzWyMz8JjPQAHEkmfVz/ZCl9
//! 5G/yrX3YjVz6Hpy/kmJJm5xUS64a5NItl+Cqbcybv+oqCdwYCIAWjpozGUauw1yZOMogHrBBcgVy
//! BSzwcAyYYD8GVnsAQU5NRrAAAAACAAAFAAA2AAA1AAD0AQAAVlA4TJgAAAAvNkANEBcw//M///Mf
//! gAfHtW2lzv0ROohRDSXEboQSqIZxxl9vm0h8HtH/CcBL98u6zlc5qamJknldK9k39I83RMm8zknN
//! 69B5VJ9Xda3NOaZTYNpEVZu2zsY640iyky4k2SCNkkHlv+skXziTkkSukknF8A1hKrWeXJ3JzmYs
//! su3HxZL5JW3ITRbAfiqFJdCqJAucyRy5Ag==" />
//!
//! [ChobitAniValue] helps to calculate left, top, right, bottom coordinates.
//!
//! # Example
//!
//! ```ignore
//! use chobitlibs::chobit_ani_value::ChobitAniValue;
//!
//! // All coordinates are calculated and saved at ChobitAniValue::new().
//! let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
//! assert_eq!(ani_value.uv_frame_width(), 0.25);
//! assert_eq!(ani_value.uv_frame_height(), 0.5);
//!
//! // The 1st frame of the 1st row.
//! ani_value.set_frame(0);
//! ani_value.set_row(0);
//! assert_eq!(
//!     ani_value.uv_frame_left_top_right_bottom(),
//!     &(0.0, 0.0, 0.25, 0.5)
//! );
//!
//! // The 2nd frame of the 1st row.
//! let frame_number = ani_value.next_frame();
//! assert_eq!(frame_number, 1);
//! assert_eq!(
//!     ani_value.uv_frame_left_top_right_bottom(),
//!     &(0.25, 0.0, 0.5, 0.5)
//! );
//!
//! // The 3rd frame of the 1st row.
//! let frame_number = ani_value.next_frame();
//! assert_eq!(frame_number, 2);
//! assert_eq!(
//!     ani_value.uv_frame_left_top_right_bottom(),
//!     &(0.5, 0.0, 0.75, 0.5)
//! );
//!
//! // The 4th frame of the 1st row.
//! let frame_number = ani_value.next_frame();
//! assert_eq!(frame_number, 3);
//! assert_eq!(
//!     ani_value.uv_frame_left_top_right_bottom(),
//!     &(0.75, 0.0, 1.0, 0.5)
//! );
//!
//! // The 1st frame of the 1st row.
//! let frame_number = ani_value.next_frame();
//! assert_eq!(frame_number, 0);
//! assert_eq!(
//!     ani_value.uv_frame_left_top_right_bottom(),
//!     &(0.0, 0.0, 0.25, 0.5)
//! );
//!
//! // The 1st frame of the 2nd row.
//! ani_value.set_frame(0);
//! ani_value.set_row(1);
//! assert_eq!(
//!     ani_value.uv_frame_left_top_right_bottom(),
//!     &(0.0, 0.5, 0.25, 1.0)
//! );
//!
//! // The 2nd frame of the 2nd row.
//! let frame_number = ani_value.next_frame();
//! assert_eq!(frame_number, 1);
//! assert_eq!(
//!     ani_value.uv_frame_left_top_right_bottom(),
//!     &(0.25, 0.5, 0.5, 1.0)
//! );
//!
//! // The 1st frame of the 2nd row.
//! let frame_number = ani_value.next_frame();
//! assert_eq!(frame_number, 0);
//! assert_eq!(
//!     ani_value.uv_frame_left_top_right_bottom(),
//!     &(0.0, 0.5, 0.25, 1.0)
//! );
//! ```

use alloc::{vec::Vec, boxed::Box};
use core::fmt;

/// Error for [ChobitAniValue]
#[derive(Debug, Clone, PartialEq)]
pub enum ChobitAniValueError {
    /// 1st argument of [ChobitAniValue::new()] is wrong.  
    /// It must be [MIN_COLUMNS] or more.
    InvalidColumns,

    /// Each element of 2nd argument of [ChobitAniValue::new()] is wrong.  
    /// It must be \[[MIN_FRAMES], 1st argument).
    InvalidFramesOfEachRow,

    /// Number of elements of 2nd argument of [ChobitAniValue::new()] is wrong.  
    /// It must be [MIN_ROWS] or more.
    InvalidRows,

    /// 3rd argument of [ChobitAniValue::new()] is worng.
    /// It must be [MIN_FRAMES_PER_SECOND] or more.
    InvalidFramesPerSecond
}

impl fmt::Display for ChobitAniValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, r#"{{"error":"ChobitAniValueError","kind":"#)?;

        match self {
            Self::InvalidColumns => {
                write!(formatter, r#""InvalidColumns""#)?;
            },

            Self::InvalidFramesOfEachRow => {
                write!(formatter, r#""InvalidFramesOfEachRow""#)?;
            },

            Self::InvalidRows => {
                write!(formatter, r#""InvalidRows""#)?;
            },

            Self::InvalidFramesPerSecond => {
                write!(formatter, r#""InvalidFramesPerSecond""#)?;
            },
        }

        write!(formatter, "}}")
    }
}

/// Utility for UV animation.
#[derive(Debug, Clone, PartialEq)]
pub struct ChobitAniValue {
    columns: usize,
    rows: usize,
    last_frame: Box<[usize]>,
    last_row: usize,
    next_frame: Box<[Box<[usize]>]>,
    prev_frame: Box<[Box<[usize]>]>,

    current_frame: usize,
    current_row: usize,

    uv_frame_width: f32,
    uv_frame_height: f32,
    uv_frame: Box<[Box<[(f32, f32, f32, f32)]>]>,

    saved_time: f32,
    seconds_per_frame: f32
}

/// Minimum number for 1st argument of [ChobitAniValue::new()].
pub const MIN_COLUMNS: usize = 1;
/// Minimum number for each element of 2nd argument of [ChobitAniValue::new()].
pub const MIN_FRAMES: usize = 1;
/// Minimum number for number of elements of 2nd argument of [ChobitAniValue::new()].
pub const MIN_ROWS: usize = 1;
/// Minimum number for 3rd argument of [ChobitAniValue::new()].
pub const MIN_FRAMES_PER_SECOND: f32 = f32::EPSILON;

impl ChobitAniValue {
    /// Generates ChobitAniValue.
    ///
    /// Each UV coordinate of all frames and some values are calculated and saved here,
    /// so it is very fast to get each value.
    ///
    /// - `columns` : Columns of UV frame. (must be 1 or more)
    /// - `frames_of_each_row` : Frames of each row of UV frame. (lenght must be 1 or more and each element must be 1 or more)
    /// - `frames_per_second` : Frames per seconds. (must be `f32::EPSILON` or more)
    /// - _Return_ : ChobitAniValue.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_ani_value::ChobitAniValue;
    ///
    /// // Columns are 4, rows are 2.
    /// // The 1st row has 4 frames.
    /// // The 2nd row has 2 frames.
    /// // | * | * | * | * |
    /// // | * | * |   |   |
    /// //
    /// // FPS is 4.0.
    /// let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
    ///
    /// // Columns are 4, because the 1st argument is 4.
    /// assert_eq!(ani_value.columns(), 4);
    ///
    /// // Rows are 2, because the length of the 2nd argument is 2.
    /// assert_eq!(ani_value.rows(), 2);
    ///
    /// // This equals the 3rd argument.
    /// assert_eq!(ani_value.frames_per_second(), 4.0);
    ///
    /// // This is reciprocal of the 3rd argument.
    /// assert_eq!(ani_value.seconds_per_frame(), 0.25);
    ///
    /// // The last frame of the 1st row is 3,
    /// // because the 1st element of the 2nd argument is 4.
    /// // (the 1st frame is 0)
    /// ani_value.set_row(0);  // the 1st row is 0.
    /// assert_eq!(ani_value.last_frame(), 3);
    ///
    /// // The last frame of the 2nd row is 1,
    /// // because the 2nd element of the 2nd argument is 2.
    /// // (the 1st frame is 0)
    /// ani_value.set_row(1);  // the 2nd row is 1.
    /// assert_eq!(ani_value.last_frame(), 1);
    /// ```
    pub fn new(
        columns: usize,
        frames_of_each_row: &[usize],
        frames_per_second: f32,
    ) -> Result<Self, ChobitAniValueError> {
        if columns < MIN_COLUMNS {
            return Err(ChobitAniValueError::InvalidColumns);
        }

        if frames_per_second < MIN_FRAMES_PER_SECOND {
            return Err(ChobitAniValueError::InvalidFramesPerSecond);
        }

        let rows = frames_of_each_row.len();

        if rows < MIN_ROWS {
            return Err(ChobitAniValueError::InvalidRows);
        }

        let mut last_frame = Vec::<usize>::with_capacity(rows);

        for &frames in frames_of_each_row {
            if (frames >= MIN_FRAMES) && (frames <= columns) {
                last_frame.push(frames - 1);
            } else {
                return Err(ChobitAniValueError::InvalidFramesOfEachRow);
            }
        }


        let next_frame = Self::gen_next_frame(columns, rows, &*last_frame);
        let prev_frame = Self::gen_prev_frame(columns, rows, &*last_frame);

        let uv_frame_width = (columns as f32).recip();
        let uv_frame_height = (rows as f32).recip();

        let uv_frame = Self::gen_uv_frame(
            columns,
            rows,
            uv_frame_width,
            uv_frame_height
        );

        Ok(Self {
            columns: columns,
            rows: rows,
            last_frame: last_frame.into_boxed_slice(),
            last_row: rows - 1,
            next_frame: next_frame,
            prev_frame: prev_frame,

            current_frame: 0,
            current_row: 0,

            uv_frame_width: uv_frame_width,
            uv_frame_height: uv_frame_height,
            uv_frame: uv_frame,

            saved_time: 0.0,
            seconds_per_frame: frames_per_second.recip()
        })
    }

    fn gen_next_frame(
        columns: usize,
        rows: usize,
        last_frame: &[usize]
    ) -> Box<[Box<[usize]>]> {
        (0..columns).map(move |col| {
            (0..rows).map(move |ro| {
                let last_frame = last_frame[ro];

                if col == last_frame {
                    0
                } else {
                    col + 1
                }
            }).collect::<Vec<usize>>().into_boxed_slice()
        }).collect::<Vec<Box<[usize]>>>().into_boxed_slice()
    }

    fn gen_prev_frame(
        columns: usize,
        rows: usize,
        last_frame: &[usize]
    ) -> Box<[Box<[usize]>]> {
        (0..columns).map(move |col| {
            (0..rows).map(move |ro| {
                let last_frame = last_frame[ro];

                if col == 0 {
                    last_frame
                } else {
                    col - 1
                }
            }).collect::<Vec<usize>>().into_boxed_slice()
        }).collect::<Vec<Box<[usize]>>>().into_boxed_slice()
    }

    fn gen_uv_frame(
        columns: usize,
        rows: usize,
        uv_frame_width: f32,
        uv_frame_height: f32
    ) -> Box<[Box<[(f32, f32, f32, f32)]>]> {
        (0..columns).map(move |col| {
            (0..rows).map(move |ro| {
                let left = (col as f32) * uv_frame_width;
                let top = (ro as f32) * uv_frame_height;
                let right = left + uv_frame_width;
                let bottom = top + uv_frame_height;

                (left, top, right, bottom)
            }).collect::<Vec<(f32, f32, f32, f32)>>().into_boxed_slice()
        }).collect::<Vec<Box<[(f32, f32, f32, f32)]>>>().into_boxed_slice()
    }

    /// Gets columns.
    ///
    /// - _Return_ : Columns.
    #[inline]
    pub fn columns(&self) -> usize {self.columns}

    /// Gets rows.
    ///
    /// - _Return_ : Rows.
    #[inline]
    pub fn rows(&self) -> usize {self.rows}

    /// Gets current frame of current row.
    ///
    /// - _Return_ : current frame.
    #[inline]
    pub fn current_frame(&self) -> usize {self.current_frame}

    /// Gets current row.
    ///
    /// - _Return_ : Current row.
    #[inline]
    pub fn current_row(&self) -> usize {self.current_row}

    /// Gets last frame of current row.
    ///
    /// - _Return_ : Last frame.
    #[inline]
    pub fn last_frame(&self) -> usize {
        debug_assert!(self.last_frame.get(self.current_row).is_some());

        unsafe {*self.last_frame.get_unchecked(self.current_row)}
    }

    /// Gets saved time for changing frame.
    ///
    /// - _Return_ : Saved time.
    #[inline]
    pub fn saved_time(&self) -> f32 {self.saved_time}

    /// Gets mutable saved time for changing frame.
    ///
    /// - _Return_ : Mutable saved time.
    #[inline]
    pub fn saved_time_mut(&mut self) -> &mut f32 {&mut self.saved_time}

    /// Gets seconds per frame.
    ///
    /// - _Return_ : Seconds per frame.
    #[inline]
    pub fn seconds_per_frame(&self) -> f32 {self.seconds_per_frame}

    /// Gets frames per second.
    ///
    /// - _Return_ : Frames per second.
    #[inline]
    pub fn frames_per_second(&self) -> f32 {self.seconds_per_frame.recip()}

    /// Gets width of UV frame. [0.0, 1.0]
    ///
    /// - _Return_ : Width of UV frame.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_ani_value::ChobitAniValue;
    ///
    /// // Columns are 4, rows are 2.
    /// // The 1st row has 4 frames.
    /// // The 2nd row has 2 frames.
    /// // | * | * | * | * |
    /// // | * | * |   |   |
    /// //
    /// // FPS is 4.0.
    /// let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
    ///
    /// // 0.25 because columns are 4.
    /// assert_eq!(ani_value.uv_frame_width(), 0.25);
    /// ```
    #[inline]
    pub fn uv_frame_width(&self) -> f32 {self.uv_frame_width}

    /// Gets height of UV frame. [0.0, 1.0]
    ///
    /// - _Return_ : height of UV frame.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_ani_value::ChobitAniValue;
    ///
    /// // Columns are 4, rows are 2.
    /// // The 1st row has 4 frames.
    /// // The 2nd row has 2 frames.
    /// // | * | * | * | * |
    /// // | * | * |   |   |
    /// //
    /// // FPS is 4.0.
    /// let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
    ///
    /// // 0.5 because rows are 2.
    /// assert_eq!(ani_value.uv_frame_height(), 0.5);
    /// ```
    #[inline]
    pub fn uv_frame_height(&self) -> f32 {self.uv_frame_height}

    /// Gets (left, top, right, bottom) of UV frame at current frame.
    ///
    /// - _Return_ : (left, top, right, bottom) of UV frame.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_ani_value::ChobitAniValue;
    ///
    /// // Columns are 4, rows are 2.
    /// // The 1st row has 4 frames.
    /// // The 2nd row has 2 frames.
    /// // | * | * | * | * |
    /// // | * | * |   |   |
    /// //
    /// // FPS is 4.0.
    /// let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
    ///
    /// // ----- The 1st row ----- //
    /// ani_value.set_row(0);
    ///
    /// // The 1st frame.
    /// ani_value.set_frame(0);
    /// assert_eq!(
    ///     ani_value.uv_frame_left_top_right_bottom(),
    ///     &(0.0, 0.0, 0.25, 0.5)
    /// );
    ///
    /// // The 2nd frame.
    /// ani_value.set_frame(1);
    /// assert_eq!(
    ///     ani_value.uv_frame_left_top_right_bottom(),
    ///     &(0.25, 0.0, 0.5, 0.5)
    /// );
    ///
    /// // The 3rd frame.
    /// ani_value.set_frame(2);
    /// assert_eq!(
    ///     ani_value.uv_frame_left_top_right_bottom(),
    ///     &(0.5, 0.0, 0.75, 0.5)
    /// );
    ///
    /// // The 4th frame.
    /// ani_value.set_frame(3);
    /// assert_eq!(
    ///     ani_value.uv_frame_left_top_right_bottom(),
    ///     &(0.75, 0.0, 1.0, 0.5)
    /// );
    ///
    /// // ----- The 2nd row ----- //
    /// ani_value.set_row(1);
    ///
    /// // The 1st frame.
    /// ani_value.set_frame(0);
    /// assert_eq!(
    ///     ani_value.uv_frame_left_top_right_bottom(),
    ///     &(0.0, 0.5, 0.25, 1.0)
    /// );
    ///
    /// // The 2nd frame.
    /// ani_value.set_frame(1);
    /// assert_eq!(
    ///     ani_value.uv_frame_left_top_right_bottom(),
    ///     &(0.25, 0.5, 0.5, 1.0)
    /// );
    /// ```
    ///
    #[inline]
    pub fn uv_frame_left_top_right_bottom(
        &self
    ) -> &(f32, f32, f32, f32) {
        debug_assert!(self.uv_frame.get(self.current_frame).is_some());

        unsafe {
            debug_assert!(self.uv_frame.get_unchecked(
                self.current_frame
            ).get(
                self.current_row
            ).is_some());

            self.uv_frame.get_unchecked(
                self.current_frame
            ).get_unchecked(
                self.current_row
            )
        }
    }

    /// Sets frames per second.
    ///
    /// - `frames_per_second` : Frames per second.
    #[inline]
    pub fn set_frames_per_second(&mut self, frames_per_second: f32) {
        self.seconds_per_frame = frames_per_second.recip();
    }

    /// Sets current frame of current row.
    ///
    /// - `frame` : Current frame.
    #[inline]
    pub fn set_frame(&mut self, frame: usize) {
        self.saved_time = 0.0;
        self.current_frame = self.last_frame().min(frame);
    }

    /// Sets current row.
    ///
    /// - `row` : Current row.
    #[inline]
    pub fn set_row(&mut self, row: usize) {
        self.saved_time = 0.0;
        self.current_row = self.rows.min(row);
        self.current_frame = 0;
    }

    #[inline]
    fn next_frame_core(&mut self) -> usize {
        debug_assert!(self.next_frame.get(self.current_frame).is_some());

        self.current_frame = unsafe {
            debug_assert!(self.next_frame.get_unchecked(
                self.current_frame
            ).get(
                self.current_row
            ).is_some());

            *self.next_frame.get_unchecked(
                self.current_frame
            ).get_unchecked(
                self.current_row
            )
        };

        self.current_frame
    }

    /// Changes current frame into next frame.
    ///
    /// If current frame is the last frame, changes it into the first frame.
    ///
    /// - _Return_ : Changed current frame.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_ani_value::ChobitAniValue;
    ///
    /// // Columns are 4, rows are 2.
    /// // The 1st row has 4 frames.
    /// // The 2nd row has 2 frames.
    /// // | * | * | * | * |
    /// // | * | * |   |   |
    /// //
    /// // FPS is 4.0.
    /// let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
    ///
    /// // ----- The 1st row ----- //
    /// ani_value.set_row(0);
    ///
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.next_frame(), 1);
    /// assert_eq!(ani_value.current_frame(), 1);
    ///
    /// assert_eq!(ani_value.next_frame(), 2);
    /// assert_eq!(ani_value.current_frame(), 2);
    ///
    /// assert_eq!(ani_value.next_frame(), 3);
    /// assert_eq!(ani_value.current_frame(), 3);
    ///
    /// assert_eq!(ani_value.next_frame(), 0);
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// // ----- The 2nd row ----- //
    /// ani_value.set_row(1);
    ///
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.next_frame(), 1);
    /// assert_eq!(ani_value.current_frame(), 1);
    ///
    /// assert_eq!(ani_value.next_frame(), 0);
    /// assert_eq!(ani_value.current_frame(), 0);
    /// ```
    #[inline]
    pub fn next_frame(&mut self) -> usize {
        self.saved_time = 0.0;
        self.next_frame_core()
    }

    #[inline]
    fn prev_frame_core(&mut self) -> usize {
        debug_assert!(self.next_frame.get(self.current_frame).is_some());

        self.current_frame = unsafe {
            debug_assert!(self.prev_frame.get_unchecked(
                self.current_frame
            ).get(
                self.current_row
            ).is_some());

            *self.prev_frame.get_unchecked(
                self.current_frame
            ).get_unchecked(
                self.current_row
            )
        };

        self.current_frame
    }

    /// Changes current frame into previous frame.
    ///
    /// If current frame is the first frame, changes it into the last frame.
    ///
    /// - _Return_ : Changed current frame.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_ani_value::ChobitAniValue;
    ///
    /// // Columns are 4, rows are 2.
    /// // The 1st row has 4 frames.
    /// // The 2nd row has 2 frames.
    /// // | * | * | * | * |
    /// // | * | * |   |   |
    /// //
    /// // FPS is 4.0.
    /// let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
    ///
    /// // ----- The 1st row ----- //
    /// ani_value.set_row(0);
    ///
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.prev_frame(), 3);
    /// assert_eq!(ani_value.current_frame(), 3);
    ///
    /// assert_eq!(ani_value.prev_frame(), 2);
    /// assert_eq!(ani_value.current_frame(), 2);
    ///
    /// assert_eq!(ani_value.prev_frame(), 1);
    /// assert_eq!(ani_value.current_frame(), 1);
    ///
    /// assert_eq!(ani_value.prev_frame(), 0);
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// // ----- The 2nd row ----- //
    /// ani_value.set_row(1);
    ///
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.prev_frame(), 1);
    /// assert_eq!(ani_value.current_frame(), 1);
    ///
    /// assert_eq!(ani_value.prev_frame(), 0);
    /// assert_eq!(ani_value.current_frame(), 0);
    /// ```
    #[inline]
    pub fn prev_frame(&mut self) -> usize {
        self.saved_time = 0.0;
        self.prev_frame_core()
    }

    /// Saves `dt` and changes current frame into next frame by seconds per frame.
    ///
    /// - `dt` : Delta time. (seconds)
    /// - _Return_ : Current frame.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_ani_value::ChobitAniValue;
    ///
    /// // Columns are 4, rows are 2.
    /// // The 1st row has 4 frames.
    /// // The 2nd row has 2 frames.
    /// // | * | * | * | * |
    /// // | * | * |   |   |
    /// //
    /// // FPS is 4.0.
    /// let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
    ///
    /// // FPS is 4.0, so after 0.25 seconds, frame goes next.
    /// ani_value.set_frame(0);  // total 0.0 seconds.
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.elapse(0.2), 0);  // total 0.2 seconds.
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.elapse(0.2), 1);  // total 0.4 seconds.
    /// assert_eq!(ani_value.current_frame(), 1);
    ///
    /// assert_eq!(ani_value.elapse(0.2), 2);  // total 0.6 seconds.
    /// assert_eq!(ani_value.current_frame(), 2);
    ///
    /// // Reset.
    /// ani_value.set_frame(0);  // total 0.0 seconds.
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.elapse(0.6), 2);  // total 0.6 seconds.
    /// assert_eq!(ani_value.current_frame(), 2);
    /// ```
    #[inline]
    pub fn elapse(&mut self, dt: f32) -> usize {
        self.saved_time += dt;

        let mut ret: usize = self.current_frame;

        while self.saved_time >= self.seconds_per_frame {
            ret = self.next_frame_core();

            self.saved_time -= self.seconds_per_frame;
        }

        ret
    }

    /// Saves `dt` and changes current frame into previous frame by seconds per frame.
    ///
    /// - `dt` : Delta time. (seconds)
    /// - _Return_ : Current frame.
    ///
    /// ```ignore
    /// use chobitlibs::chobit_ani_value::ChobitAniValue;
    ///
    /// // Columns are 4, rows are 2.
    /// // The 1st row has 4 frames.
    /// // The 2nd row has 2 frames.
    /// // | * | * | * | * |
    /// // | * | * |   |   |
    /// //
    /// // FPS is 4.0.
    /// let mut ani_value = ChobitAniValue::new(4, &[4, 2], 4.0).unwrap();
    ///
    /// // FPS is 4.0, so after 0.25 seconds, frame goes previous.
    /// ani_value.set_frame(0);  // total 0.0 seconds.
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.elapse_inv(0.2), 0);  // total 0.2 seconds.
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.elapse_inv(0.2), 3);  // total 0.4 seconds.
    /// assert_eq!(ani_value.current_frame(), 3);
    ///
    /// assert_eq!(ani_value.elapse_inv(0.2), 2);  // total 0.6 seconds.
    /// assert_eq!(ani_value.current_frame(), 2);
    ///
    /// // Reset.
    /// ani_value.set_frame(0);  // total 0.0 seconds.
    /// assert_eq!(ani_value.current_frame(), 0);
    ///
    /// assert_eq!(ani_value.elapse_inv(0.6), 2);  // total 0.6 seconds.
    /// assert_eq!(ani_value.current_frame(), 2);
    /// ```
    #[inline]
    pub fn elapse_inv(&mut self, dt: f32) -> usize {
        self.saved_time += dt;

        let mut ret: usize = self.current_frame;

        while self.saved_time >= self.seconds_per_frame {
            ret = self.prev_frame_core();

            self.saved_time -= self.seconds_per_frame;
        }

        ret
    }
}
