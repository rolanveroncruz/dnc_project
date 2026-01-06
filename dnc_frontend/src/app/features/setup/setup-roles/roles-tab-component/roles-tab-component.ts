import {Component, inject, Input} from '@angular/core';
import {Role} from '../../../../api_services/roles-and-permissions-service';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
import {
  GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {AddEditRoles} from '../add-edit-roles/add-edit-roles';
import {MatDialog} from '@angular/material/dialog';

@Component({
  selector: 'app-roles-tab-component',
  imports: [
    GenericDataTableComponent
  ],
  templateUrl: './roles-tab-component.html',
  styleUrl: './roles-tab-component.scss',
})
export class RolesTabComponent {
  @Input() roles: Role[] | null = null;
  roleColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'name', label: 'Name'},
    {key: 'description', label: 'Description'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On'},
  ];

  private dialog = inject(MatDialog);
  constructor() { }
  openRoleRowDialog(row:any){
    console.log("In openRoleRowDialog():",row);
    const ref = this.dialog.open(AddEditRoles, {
      data:row,
      width: '720px',
      maxWidth: '95vw',
      disableClose : false,
    });
    ref.afterClosed().subscribe(result => {
      console.log('The dialog was closed');
      if (!result) return;
    });
  }

}
