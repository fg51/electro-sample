cable 静電容量
====

##
導体 - 絶縁体 - 導体 間で静電容量が発生する。

例:
* 導体 - 被覆 - シールド
* (芯線1 - 被覆1) - (被覆2 - 芯線2)


## 等価回路
 ```
 - L + L -
     |
     C
     |
     G
 ```


\\[\\begin{eqnarray}
C = \frac{q}{V_{AB}}
\\end{eqnarray}\\]

* q: 単位長さ当たりの電荷
* VAB: 電位差


\\[\\begin{eqnarray}
V_{AB} = \int_{a}^{b} E(r) dr
\\end{eqnarray}\\]

* E(r): 電界の強さ
* a: 導体半径
* b: 導体半径


\\[\\begin{eqnarray}
q = \epsilon E(r_0) S(r_0)
\\end{eqnarray}\\]

* \\( \epsilon E(r0) \\) : 電束密度
* \\( \epsilon \\) : 導体の誘電率
* S(r0): 円筒側面の面積


\\[\\begin{eqnarray}
E(r0) = \frac{q}{\epsilon S(r_0)}
\\end{eqnarray}\\]

\\[\\begin{eqnarray}
S(r0) = 2 \pi r_0
\\end{eqnarray}\\]


よって, 一般化すると
\\[\\begin{eqnarray}
E(r) = \frac{q}{2 pi r \epsilon}
\\end{eqnarray}\\]


したがって，

\\[\\begin{eqnarray}
V_{AB} &=& \int_{a}^{b} \frac{q}{2 pi r \epsilon} dr  \\\\
  &=& \frac{q}{2 pi \epsilon} \int_{a}^{b} \frac{dr}{r}  \\\\
  &=& \frac{q}{2 pi \epsilon} log(\frac{b}{a})
\\end{eqnarray}\\]


例：
b/a = 1.5, 比誘電率=2.3, 真空誘電率 \\( \epsilon_0 = 8.8853E-12 [F/m] \\)

\\[\\begin{eqnarray}
C = \frac{2 \pi \times 2.3 \epsilon_0}{log(1.5)} = 3.16 E-10 [F/m]
\\end{eqnarray}\\]


