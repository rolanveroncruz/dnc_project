import { Component } from '@angular/core';
import {GenericDataTableComponent} from '../../../components/generic-data-table-component/generic-data-table-component';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';

@Component({
  selector: 'app-clinic-capabilities',
  imports: [
    GenericDataTableComponent
  ],
  templateUrl: './clinic-capabilities.html',
  styleUrl: './clinic-capabilities.scss',
})
export class ClinicCapabilities {
// 1. Define your data
  users = [
    { id: 1, name: 'Ada', role: 'Admin', status: 'Active' },
    { id: 2, name: 'Bob', role: 'Staff', status: 'Disabled' },
    // ...
  ];

  // 2. Define your columns
  columns: TableColumn[] = [
    { key: 'id', label: 'ID' },
    { key: 'name', label: 'Full Name' },
    { key: 'role', label: 'User Role' },
    { key: 'status', label: 'Account Status' }
  ];

  // 3. Define which fields get a dropdown filter
  filterKeys = ['role', 'status'];
}
