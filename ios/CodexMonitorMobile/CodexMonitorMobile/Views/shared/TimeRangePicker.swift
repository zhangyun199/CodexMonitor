import SwiftUI
import CodexMonitorModels

struct TimeRangePicker: View {
    @Binding var selection: LifeTimeRange

    var body: some View {
        Picker("Range", selection: $selection) {
            Text("Today").tag(LifeTimeRange.today)
            Text("Week").tag(LifeTimeRange.week)
            Text("Month").tag(LifeTimeRange.month)
            Text("Life").tag(LifeTimeRange.lifetime)
        }
        .pickerStyle(.segmented)
    }
}
