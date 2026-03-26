import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {map, Observable} from 'rxjs';
import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';
import {MasterListMemberLookupResponse} from './master-list-members-service';

export interface MemberServicesCountsSummary {
    dental_service_id: number,
    dental_service_name: string,
    dental_service_type_id: number,
    counts_allowed: number,
    counts_used: number,
    has_pending: boolean,
    conflict_date: Date | null,

}
@Injectable({
  providedIn: 'root',
})
export class MemberServicesCountsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }
    private readonly baseUrl = `${environment.apiBaseUrl}/api`;

    getMemberServicesCountsSummary(dentistId: number):Observable<MemberServicesCountsSummary[]>{
        return this.http.get<MemberServicesCountsSummary[]>(`${this.baseUrl}/master_list_members/${dentistId}/service_counts`,
            { headers:this.authHeaders()});
    }
}
