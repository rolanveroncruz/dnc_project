import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {Role, RolesAndPermissionsService} from '../../../../api_services/roles-and-permissions-service';
import {TableColumn} from '../../../../components/generic-data-table-component/table-interfaces';
import {
  GenericDataTableComponent
} from '../../../../components/generic-data-table-component/generic-data-table-component';
import {AddEditRoles} from '../add-edit-roles/add-edit-roles';
import {MatDialog} from '@angular/material/dialog';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';

type LoadState = 'loading' | 'loaded' | 'error';


@Component({
  selector: 'app-roles-tab-component',
  imports: [
    GenericDataTableComponent
  ],
  templateUrl: './roles-tab-component.html',
  styleUrl: './roles-tab-component.scss',
})
export class RolesTabComponent implements OnInit{

  roles_state = signal<LoadState>('loading');
  roles = signal<Role[] | null>(null);

  private destroyRef = inject(DestroyRef);
  roleColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'name', label: 'Name'},
    {key: 'description', label: 'Description'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On', cellTemplateKey: 'datetime'},
  ];

  private dialog = inject(MatDialog);
  constructor(private roles_and_permission_Service: RolesAndPermissionsService) { }

  ngOnInit(): void {
    this.load_roles();
    }

  load_roles() {
    console.log("In load_roles()");
    this.roles_state.set('loading');
    this.roles_and_permission_Service.getRoles()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.roles.set(processRoles(res.items));
          this.roles_state.set('loaded');
          console.log("In load(), roles:",this.roles());
        },
        error: (err) => {
          console.log("In SetupRoles:getRoles():", err);
          this.roles_state.set('error');
        }
      });
  }
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
function processRoles(roles:Array<Role>):Role[] {
  let result: Role[] = []
  for (const role of roles) {
    const new_role: Role = {
      id: role.id,
      name: role.name,
      description: role.description,
      active: role.active,
      last_modified_by: role.last_modified_by,
      last_modified_on: role.last_modified_on
    };
    result.push(new_role);
  }
  return result;
}
