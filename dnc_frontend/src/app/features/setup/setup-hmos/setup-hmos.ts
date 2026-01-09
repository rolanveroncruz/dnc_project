import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';
import {GenericDataTableComponent} from "../../../components/generic-data-table-component/generic-data-table-component";
import {MatCard, MatCardContent, MatCardHeader, MatCardSubtitle, MatCardTitle} from "@angular/material/card";
import {HMO, HMOService} from '../../../api_services/hmoservice';
import {TableColumn} from '../../../components/generic-data-table-component/table-interfaces';
import {takeUntilDestroyed} from '@angular/core/rxjs-interop';
import {Router} from '@angular/router';

type LoadState = 'loading' | 'loaded' | 'error';


@Component({
  selector: 'app-setup-hmos',
  imports: [
    MatCard,
    MatCardContent,
    MatCardHeader,
    MatCardSubtitle,
    MatCardTitle,
    GenericDataTableComponent
  ],
  templateUrl: './setup-hmos.html',
  styleUrl: './setup-hmos.scss',
})
export class SetupHMOs implements OnInit{
  hmos = signal<HMO[] | null>(null);
  state = signal<LoadState>('loading');
  errorMsg = signal<string | null>(null);
  private destroyRef = inject(DestroyRef);


  HMOColumns: TableColumn[] = [
    {key: 'id', label: 'ID'},
    {key: 'short_name', label: 'Short Name'},
    {key: 'long_name', label: 'Long Name'},
    {key: 'address', label: 'Address'},
    {key: 'tax_account_number', label: 'TIN'},
    {key: 'contact_numbers', label: 'Contact Numbers'},
    {key: 'last_endorsement_date', label: 'Last Endorsement Date'},
    {key: 'last_collection_date', label: 'Last Collection Date'},
    {key: 'last_modified_by', label: 'Last Modified By'},
    {key: 'last_modified_on', label: 'Last Modified On', cellTemplateKey: 'date'},
  ];

  constructor(private hmoService: HMOService, private router: Router) {

  }

  ngOnInit(): void {
    this.load_hmos();
  }
  load_hmos(){
    this.state.set('loading');
    this.errorMsg.set(null)
    this.hmoService.getHMOs()
      .pipe(takeUntilDestroyed(this.destroyRef))
      .subscribe({
        next: (res) => {
          this.hmos.set(res.items);
          this.state.set('loaded');
        },
        error: (err) => {
          console.log("In load(), failed to load users", err);
          this.errorMsg.set("In load(), Failed to load users");
          this.state.set('error');
        }
      })
  }
  navigate_to_hmo_detail(hmo:HMO){
    this.router.navigate(['/main/setup/hmos/', hmo.id]).then();
  }

}
