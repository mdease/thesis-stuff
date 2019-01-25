import numpy as np
import matplotlib.pyplot as plt

"""
Apps
"""

def autolabel(rects, xpos='center', stype=False):
    """
    Attach a text label above each bar in *rects*, displaying its height.

    *xpos* indicates which side to place the text w.r.t. the center of
    the bar. It can be one of the following {'center', 'right', 'left'}.
    """

    xpos = xpos.lower()  # normalize the case of the parameter
    ha = {'center': 'center', 'right': 'left', 'left': 'right'}
    offset = {'center': 0.5, 'right': 0.57, 'left': 0.43}  # x_txt = x + w*off

    for rect in rects:
        height = rect.get_height()

        if stype and height < 740:
            height += 742

        ax.text(rect.get_x() + rect.get_width()*offset[xpos], 1.01*height,
                '{}'.format(height), ha=ha[xpos], va='bottom')

no_avid_means, no_avid_std = (3, 14.01, 1.424), (0, 0.1071, 0.1357)
avid_means, avid_std = (11, 14.13, 6.847), (0, 0.1801, 0.1586)

ind = np.arange(len(no_avid_means))  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, no_avid_means, width, yerr=no_avid_std,
                color='Orange', label='Without AVID')
rects2 = ax.bar(ind + width/2, avid_means, width, yerr=avid_std,
                color='SkyBlue', label='With AVID')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Number of clock ticks')
ax.set_title('System performance in number of clock ticks')
ax.set_xticks(ind)
ax.set_xticklabels(('App Header', 'Bluetooth', 'Syscall'))
ax.legend()

autolabel(rects1, "left")
autolabel(rects2, "right")

#################

no_avid_means = 3
avid_means = 11

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, no_avid_means, width,
                color='Orange', label='Without AVID')
rects2 = ax.bar(ind + width/2, avid_means, width,
                color='SkyBlue', label='With AVID')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Number of clock ticks')
ax.set_title('Application load performance in number of clock ticks')
ax.set_xticks([])
ax.legend()

autolabel(rects1, "center")
autolabel(rects2, "center")

#################

no_avid_means, no_avid_std = 14.01, 0.1071
avid_means, avid_std = 14.13, 0.1801

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, no_avid_means, width, yerr=no_avid_std,
                color='Orange', label='Without AVID')
rects2 = ax.bar(ind + width/2, avid_means, width, yerr=avid_std,
                color='SkyBlue', label='With AVID')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Number of clock ticks')
ax.set_title('Bluetooth advertising performance in number of clock ticks')
ax.set_xticks([])
ax.set_ylim(13.5, 14.5)
ax.legend(loc=4)

autolabel(rects1, "left")
autolabel(rects2, "right")

#################

no_avid_means, no_avid_std = 1.424, 0.1357
avid_means, avid_std = 6.847, 0.1586

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, no_avid_means, width, yerr=no_avid_std,
                color='Orange', label='Without AVID')
rects2 = ax.bar(ind + width/2, avid_means, width, yerr=avid_std,
                color='SkyBlue', label='With AVID')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Number of clock ticks')
ax.set_title('System call performance in number of clock ticks')
ax.set_xticks([])
ax.legend()

autolabel(rects1, "left")
autolabel(rects2, "right")

#################

no_avid_means, no_avid_std = 1.424, 0.1357
parse_means, parse_std = 6.28, 0.1271
avid_means, avid_std = 6.847, 0.1586

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width, no_avid_means, width, yerr=no_avid_std,
                color='Orange', label='Without AVID')
rects2 = ax.bar(ind, parse_means, width, yerr=parse_std,
                color='Green', label='With AVID parsing only')
rects3 = ax.bar(ind + width, avid_means, width, yerr=avid_std,
                color='SkyBlue', label='With AVID')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Number of clock ticks')
ax.set_title('System call performance in number of clock ticks')
ax.set_xticks([])
ax.legend()

autolabel(rects1, "right")
autolabel(rects2, "right")
autolabel(rects3, "right")

"""
Compilation
"""

means, std = (916.08, 915.91, 912.31, 922.64, 923.24), (16.484, 72.207, 15.073, 19.420, 20.883)

ind = np.arange(len(means))  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects = ax.bar(ind, means, width, yerr=std,
                color='SkyBlue')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in milliseconds')
ax.set_xlabel('Number of struct fields')
ax.set_title('Compile time versus struct size')
ax.set_xticks(ind)
ax.set_ylim(800, 1000)
ax.set_xticklabels(('1', '2', '3', '4', '5'))
ax.set_xbound(-0.5, 4.6)

autolabel(rects, "right")

#################

u16_mean, u16_std = 912.31, 15.073
diff_mean, diff_std = 917.28, 16.542

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, u16_mean, width, yerr=u16_std,
                color='IndianRed', label='Same types')
rects2 = ax.bar(ind + width/2, diff_mean, width, yerr=diff_std,
                color='SkyBlue', label='Different types')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in milliseconds')
ax.set_title('Compile time by types of struct fields')
ax.set_xticks([])
ax.set_ylim(800, 1000)
ax.legend()

autolabel(rects1, "left")
autolabel(rects2, "right")

#################

no_p_mean, no_p_std = 742.16, 9.6563
p_mean, p_std = 912.31, 15.073

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, no_p_mean, width, yerr=no_p_std,
                color='Orange', label='Without AVID')
rects2 = ax.bar(ind + width/2, p_mean, width, yerr=p_std,
                color='SkyBlue', label='With AVID')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in milliseconds')
ax.set_title('Compile time with and without enabling AVID')
ax.set_xticks([])
ax.set_ylim(600, 1000)
ax.legend()

autolabel(rects1, "left")
autolabel(rects2, "right")

#################

no_avid_means = (742.16, 741.98)
avid_means = (170.15, 295.32)
no_avid_std = (9.6563, 10.207)
avid_std = (15.073, 37.899)

ind = np.arange(2)    # the x locations for the groups
width = 0.35       # the width of the bars: can also be len(x) sequence

fig, ax = plt.subplots()
rects1 = ax.bar(ind, no_avid_means, width, yerr=no_avid_std,
                color='Orange', label='Without AVID')
rects2 = ax.bar(ind, avid_means, width, yerr=avid_std,
                bottom=no_avid_means, color='SkyBlue', label='With AVID')

plt.ylabel('Time in milliseconds')
plt.title('Compile time by struct type')
plt.xticks(ind, ('Simple', 'Nested'))
plt.legend()

autolabel(rects1, "right", True)
autolabel(rects2, "right", True)

"""
Parse
"""

means, std = (1442.7, 2555.7, 3542.9, 3942.9, 4731.2), (271.96, 341.04, 1921.4, 702.84, 832.11)

ind = np.arange(len(means))  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects = ax.bar(ind, means, width, yerr=std,
                color='SkyBlue')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in nanoseconds')
ax.set_xlabel('Number of struct fields')
ax.set_title('Parse time versus struct size')
ax.set_xticks(ind)
ax.set_xticklabels(('1', '2', '3', '4', '5'))
ax.set_xbound(-0.5, 4.6)

autolabel(rects, "right")

#################

u16_mean, u16_std = 3542.9, 1921.4
diff_mean, diff_std = 3167.8, 571.02

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, u16_mean, width, yerr=u16_std,
                color='IndianRed', label='Same types')
rects2 = ax.bar(ind + width/2, diff_mean, width, yerr=diff_std,
                color='SkyBlue', label='Different types')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in nanoseconds')
ax.set_title('Parse time by types of struct fields')
ax.set_xticks([])
ax.legend()

autolabel(rects1, "left")
autolabel(rects2, "right")

#################

simple_mean, simple_std = 4731.2, 832.11
nested_mean, nested_std = 4633.2, 844.59

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, simple_mean, width, yerr=simple_std,
                color='IndianRed', label='Simple')
rects2 = ax.bar(ind + width/2, nested_mean, width, yerr=nested_std,
                color='SkyBlue', label='Nested')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in nanoseconds')
ax.set_title('Parse time by struct type')
ax.set_xticks([])
ax.legend()

autolabel(rects1, "left")
autolabel(rects2, "right")

"""
Create
"""

means, std = (390.98, 551.30, 791.54, 854.58, 1021.6), (155.66, 145.04, 248.06, 155.38, 166.92)

ind = np.arange(len(means))  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects = ax.bar(ind, means, width, yerr=std,
                color='SkyBlue')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in nanoseconds')
ax.set_xlabel('Number of struct fields')
ax.set_title('Instantiation time versus struct size')
ax.set_xticks(ind)
ax.set_xticklabels(('1', '2', '3', '4', '5'))
ax.set_xbound(-0.5, 4.6)

autolabel(rects, "right")

#################

u16_mean, u16_std = 791.54, 248.06
diff_mean, diff_std = 720.5, 179.99

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, u16_mean, width, yerr=u16_std,
                color='IndianRed', label='Same types')
rects2 = ax.bar(ind + width/2, diff_mean, width, yerr=diff_std,
                color='SkyBlue', label='Different types')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in nanoseconds')
ax.set_title('Instantiation time by types of struct fields')
ax.set_xticks([])
ax.legend()

autolabel(rects1, "left")
autolabel(rects2, "right")

#################

simple_mean, simple_std = 1021.6, 166.92
nested_mean, nested_std = 4480.97, 1470.2

ind = np.arange(1)  # the x locations for the groups
width = 0.35  # the width of the bars

fig, ax = plt.subplots()
rects1 = ax.bar(ind - width/2, simple_mean, width, yerr=simple_std,
                color='IndianRed', label='Simple')
rects2 = ax.bar(ind + width/2, nested_mean, width, yerr=nested_std,
                color='SkyBlue', label='Nested')

# Add some text for labels, title and custom x-axis tick labels, etc.
ax.set_ylabel('Time in nanoseconds')
ax.set_title('Instantiation time by struct type')
ax.set_xticks([])
ax.legend()

autolabel(rects1, "left")
autolabel(rects2, "right")

plt.show()
