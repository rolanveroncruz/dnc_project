// src/app/api_services/state-service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface PageResponse<T> {
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
  items: T[];
}

export interface StateRow {
  id: number;
  name: string;
  region_id: number;
}

export interface CreateStateRequest {
  name: string;
  region_id: number;
}

export interface PatchStateRequest {
  name?: string | null;
  region_id?: number | null;
}

@Injectable({ providedIn: 'root' })
export class StateService {
  private readonly http = inject(HttpClient);

  // backend routes:
  // GET    /api/states?page=&page_size=&region_id=
  // GET    /api/states/:id
  // POST   /api/states
  // PATCH  /api/states/:id
  private readonly baseUrl = `${environment.apiUrl}/states`;

  getStates(opts?: { page?: number; pageSize?: number; regionId?: number }): Observable<PageResponse<StateRow>> {
    let params = new HttpParams();

    if (opts?.page != null) params = params.set('page', String(opts.page));
    if (opts?.pageSize != null) params = params.set('page_size', String(opts.pageSize));
    if (opts?.regionId != null) params = params.set('region_id', String(opts.regionId));

    return this.http.get<PageResponse<StateRow>>(this.baseUrl, { params });
  }

  getStateById(id: number): Observable<StateRow> {
    return this.http.get<StateRow>(`${this.baseUrl}/${id}`);
  }

  createState(payload: CreateStateRequest): Observable<StateRow> {
    return this.http.post<StateRow>(this.baseUrl, payload);
  }

  patchState(id: number, payload: PatchStateRequest): Observable<StateRow> {
    return this.http.patch<StateRow>(`${this.baseUrl}/${id}`, payload);
  }
}

