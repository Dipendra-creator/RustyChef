ft <= right {
        let mut mid = left + ((right - left) / 2);

        if array[mid] == element{
            return mid;
        }
        else if element < array[mid] {
            right = mid - 1;
        }
        else {
            left = mid + 1;
        }
    }