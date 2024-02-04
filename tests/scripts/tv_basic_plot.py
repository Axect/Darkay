import pandas as pd
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('../data/tv_basic.parquet')

# Prepare Data to Plot
t = df['t'][:]
p_surv = df['p_surv'][:]
p_surv_tv = df['p_surv_tv'][:]

# Plot params
pparam = dict(
    xlabel = r'$t$',
    ylabel = r'$P_{\rm surv}$',
    xscale = 'log',
    yscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(t, p_surv, label=r'$P_{\rm surv}$')
    ax.plot(t, p_surv_tv, label=r'$P_{\rm surv}^{TV}$')
    ax.legend()
    fig.savefig('tv_basic.png', dpi=600, bbox_inches='tight')
