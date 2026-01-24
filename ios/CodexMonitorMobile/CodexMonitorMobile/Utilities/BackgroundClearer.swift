import SwiftUI
import UIKit

/// Utility to recursively clear UIKit backgrounds for transparent SwiftUI views
struct BackgroundClearer {

    /// Recursively clears background colors on a view and all its subviews
    static func clearBackgrounds(in view: UIView) {
        view.backgroundColor = .clear

        // Special handling for collection views (iOS 16+ List)
        if let collectionView = view as? UICollectionView {
            collectionView.backgroundView = nil
            collectionView.backgroundColor = .clear
        }

        // Special handling for table views (older List)
        if let tableView = view as? UITableView {
            tableView.backgroundView = nil
            tableView.backgroundColor = .clear
        }

        // Special handling for scroll views
        if let scrollView = view as? UIScrollView {
            scrollView.backgroundColor = .clear
        }

        // Recurse into subviews
        for subview in view.subviews {
            clearBackgrounds(in: subview)
        }
    }

    /// Clears backgrounds on a UISplitViewController and all its child view controllers
    static func clearSplitViewBackgrounds(_ splitVC: UISplitViewController) {
        splitVC.view.backgroundColor = .clear

        // Clear all view controllers
        for vc in splitVC.viewControllers {
            vc.view.backgroundColor = .clear
            clearBackgrounds(in: vc.view)
        }

        // Clear primary column
        if let primary = splitVC.viewController(for: .primary) {
            primary.view.backgroundColor = .clear
            clearBackgrounds(in: primary.view)
        }

        // Clear supplementary column (content)
        if let supplementary = splitVC.viewController(for: .supplementary) {
            supplementary.view.backgroundColor = .clear
            clearBackgrounds(in: supplementary.view)
        }

        // Clear secondary column (detail)
        if let secondary = splitVC.viewController(for: .secondary) {
            secondary.view.backgroundColor = .clear
            clearBackgrounds(in: secondary.view)
        }
    }

    /// Clears backgrounds on a UITabBarController
    static func clearTabBarBackgrounds(_ tabBarVC: UITabBarController) {
        tabBarVC.view.backgroundColor = .clear

        for vc in tabBarVC.viewControllers ?? [] {
            vc.view.backgroundColor = .clear
            clearBackgrounds(in: vc.view)
        }
    }
}
